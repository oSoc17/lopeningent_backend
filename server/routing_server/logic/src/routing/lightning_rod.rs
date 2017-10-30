/// The heart of the routing algorithm.

use graph::{Path, AnnotatedPath};
use data::ServingModel;

use graph::dijkstra::DijkstraBuilder;
use graph::dijkstra::{DijkstraControl, Ending, SingleAction};
use graph::dijkstra::into_annotated_nodes;
use graph::Majorising;
use graph::NodeID;

use database::Tags;
use database::TagModifier;
use annotated::{PoiNode, AnnotatedEdge};

use newtypes::{Location, Located};
use newtypes::ToF64;
use newtypes::Km;

use std::f64;

use std::sync::atomic::Ordering;

use vec_map::VecMap;

use na;

use util;
use util::selectors::Selector;

use consts::*;
use super::util::Metadata;
use super::error::RoutingError;

/// Structure for computing the length of a route.
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Distance{
    /// Major poison value
    pub major_value : f64,
    /// Minor poison value
    pub minor_value : f64,
    /// The actual length
    pub actual_length : f64,
    /// Whether the path goes through an illegal node (turnaround)
    pub illegal_node_hits : f64,
    /// Modifier for subsequent edges
    pub node_potential : f64,
    /// Tracks the total poi hits.
    pub potential_track : f64,
}

impl Distance {
    /// New
    pub fn new(values : (f64, f64, f64, f64, f64, f64)) -> Distance {
        Distance {
            major_value : values.0,
            minor_value : values.1,
            actual_length : values.2,
            illegal_node_hits : values.3,
            node_potential : values.4,
            potential_track : values.5
        }
    }

    /// Create a default distance
    pub fn def() -> Distance {
        let mut res = Self::default();
        res.node_potential = 1.0;
        res
    }

    /// Get the actual length.
    pub fn get_length(&self) -> f64 {
        self.actual_length
    }

    /// Get the potential of this node
    pub fn get_potential(&self) -> f64 {
        self.major_value
    }

    /// Used for Majorising.
    fn as_majorising(&self) -> (f64, f64, f64) {
        (self.major_value, self.minor_value, self.illegal_node_hits)//, self.node_potential)
    }
}


impl Majorising for Distance {
    fn majorises(&self, other : &Self) -> bool {
        self.as_majorising().majorises(&other.as_majorising())
    }
}

/// Poisoner trait, for routing.
trait Poisoned {
    fn poison(&self, pos : &na::Vector3<f64>) -> f64;
}

impl Poisoned for () {
    fn poison(&self, _ : &na::Vector3<f64>) -> f64 {
        1.0
    }
}
#[allow(unused)]
struct PoisonMiddle {
    midpoint : Location,
    size : f64,
}

impl PoisonMiddle {
    #[allow(unused)]
    fn new(start : &Location, end : &Location) -> PoisonMiddle {
        PoisonMiddle {
            midpoint : Location::average(start, &Location::average(start, end)),
            size : util::distance::distance_lon_lat(start, end, Km::from_f64(EARTH_RADIUS / 2.0)).to_f64(),
        }
    }
}
/*
impl Poisoned for PoisonMiddle {
    fn poison(&self, pos : &na::Vector3<f64>) -> f64 {
        let value = self.size - util::distance::distance_lon_lat(location, &self.midpoint, Km::from_f64(EARTH_RADIUS)).to_f64();
        if value > 0.0 {
            (value / self.size * (200.0f64).ln()).exp()
        } else {
            1.0
        }
    }
}
*/
/// Simple poisoner, that computes the distance to an infinite line and exponentiates the result.
pub struct PoisonLine {
    start : na::Vector3<f64>,
    end : na::Vector3<f64>,
    radius : f64,
    size : f64,
    maximum_ln : f64,
}

impl PoisonLine {
    /// Create a new PoisonLine from two endpoints, a size factor, and a treshold.
    pub fn new(start : &Location, end : &Location, factor : f64, maximum : f64) -> PoisonLine {
        PoisonLine {
            start : start.as_3d(),
            end : end.as_3d(),
            radius : EARTH_RADIUS,
            size : util::distance::distance_lon_lat(start, end, Km::from_f64(EARTH_RADIUS)).to_f64() * factor,
            maximum_ln : maximum.ln(),
        }
    }
}

impl Poisoned for PoisonLine {
    fn poison(&self, pos : &na::Vector3<f64>) -> f64 {
        let value_vec = (pos - self.start).cross(&(self.end - self.start));
        let value = self.size - self.radius * value_vec.norm().sqrt();
        if value > 0.0 {
            let n = value / self.size * self.maximum_ln;
            n.exp()
        } else {
            1.0
        }
    }
}

struct RodController<'a, P : Poisoned, M : TagModifier + 'a> {
    max_length : f64,
    poisoner_large : P,
    poisoner_small : P,
    endings : VecMap<Distance>,
    closing : bool,
    modifier : &'a M,
    point_to_skip : Option<NodeID>,
}

impl<'a, P : Poisoned, M : TagModifier + 'a> RodController<'a, P, M> {
    fn enjoyment(&self, tags : &Tags) -> f64 {
        -self.modifier.tag_modifier(tags)
    }

    fn annotate(&self, edge : &AnnotatedEdge, potential : f64) -> Distance {
        let t = edge.dist.to_f64();
        let mut next_potential = (potential - 1.0) * (-t * FALLOFF).exp() + 1.0;
        let p_l = self.poisoner_large.poison(&edge.average);
        let p_s = self.poisoner_small.poison(&edge.average);
        let e = if self.endings.get(edge.edge.from_node as usize).is_some()
                  && self.endings.get(edge.edge.to_node as usize).is_some() {
                     (M::tag_bounds().1 / next_potential).ln() / DILUTE_FAVOURITE
                } else {
                    self.enjoyment(&edge.edge.tags)
                };
        if e != 0.0 {
            next_potential *= (e * DILUTE_FAVOURITE).exp();
            let (min, max) = M::tag_bounds();
            if next_potential > max {
                next_potential = max;
            }
            if next_potential < min {
                next_potential = min;
            }
        }
        let hit_illegal_node = if Some(edge.edge.to_node) == self.point_to_skip {1.0} else {0.0};
        let n_p = next_potential;
        let random_factor = edge.hits.load(Ordering::Relaxed) as f64 + 20.0;
        let random_factor = random_factor * random_factor * util::selectors::get_random(0.1, 1.0);
        Distance::new((t * n_p * p_l * random_factor,  t * n_p * p_s * random_factor, t , hit_illegal_node, n_p, -e))
    }
}

impl<'a, P : Poisoned, TM : TagModifier + 'a> DijkstraControl for RodController<'a, P, TM> {
    type V = PoiNode;
    type E = AnnotatedEdge;
    type M = Distance;
    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M {
        let added = self.annotate(e, m.node_potential);
        Distance {
            major_value : m.major_value + added.major_value,
            minor_value : m.minor_value + added.minor_value,
            actual_length : m.actual_length + added.actual_length,
            illegal_node_hits : m.illegal_node_hits + added.illegal_node_hits,
            node_potential : added.node_potential,
            potential_track : m.potential_track + added.potential_track,
        }
    }
    fn filter(&self, m : &Self::M) -> bool {
        m.actual_length < self.max_length
    }
    fn hint(&self, m : &Self::M) -> u64 {
        (m.major_value * 1000000.0) as u64
    }
    fn is_ending(&self, v : &Self::V, m : &Self::M) -> Ending {
        match self.endings.get(v.node.nid as usize) {
            None => Ending::No,
            Some(dist) =>
                if m.actual_length + dist.actual_length > self.max_length * MIN_LENGTH_FACTOR
                    {Ending::Yes} else {Ending::Kinda}
        }
    }
    fn yield_on_empty(&self) -> bool {
        !self.closing
    }
    fn force_finish(&self) -> bool {
        false //self.closing
    }
    fn ignore_filter_until_ending(&self) -> bool {
        self.closing
    }
}

/// Create a shortest path tree in the graph without any poisoning.
pub fn create_field_no_poisoning(serving_model : &ServingModel, starting_node : NodeID, endings : VecMap<Distance>, metadata : &Metadata, closing : bool, skip_node : Option<NodeID>)
    -> (Vec<SingleAction<Distance>>, Vec<usize>) {
    let builder = DijkstraBuilder::new(starting_node, Distance::def());

    // simple builder.
    let rod_controller = RodController{
        max_length : metadata.requested_length.to_f64(),
        poisoner_large : (),
        poisoner_small : (),
        endings : endings,
        closing : closing,
        modifier : metadata,
        point_to_skip : skip_node,
    };
    match builder.generate_dijkstra(&serving_model.graph, &rod_controller) {
        Ok(x) => x,
        Err(e) => {warn!("An error has occurred: {}", e); (Vec::new(), Vec::new())}
    }
}

/// Create a shortest path tree using the given path to poison the result.
pub fn create_field_poison(serving_model : &ServingModel, starting_node : NodeID, endings : VecMap<Distance>, metadata : &Metadata, closing : bool, skip_node : Option<NodeID>, poison_path : &Path)
    -> (Vec<SingleAction<Distance>>, Vec<usize>) {
    // simple builder
    let builder = DijkstraBuilder::new(starting_node, Distance::def());
    let large_random = util::selectors::get_random(CONFIG.min, CONFIG.max);
    let small_random = large_random - CONFIG.increase;//util::selectors::get_random(0.3, 0.5);

    // Create the poisoner.
    let location_from = &serving_model.graph.get(poison_path.first()).unwrap().located();
    let location_to = &serving_model.graph.get(poison_path.last()).unwrap().located();
    let min_distance = util::distance::distance_lon_lat(location_from, location_to, Km::from_f64(EARTH_RADIUS));
    let req_length = if closing {
        metadata.requested_length
    } else {
        metadata.requested_length - super::util::path_length(poison_path, &serving_model.graph)
    };

    let min_distance = if req_length.to_f64() < min_distance.to_f64() * 1.2 {
        min_distance * 1.2
    } else {
        req_length
    };

    // create the controller.
    let rod_controller = RodController {
        max_length : min_distance.to_f64(),
        poisoner_large : PoisonLine::new(location_from, location_to,
        large_random, util::selectors::get_random(CONFIG.min_lin, CONFIG.max_lin)),
        poisoner_small : PoisonLine::new(location_from, location_to,
        small_random, util::selectors::get_random(CONFIG.min_lin, CONFIG.max_lin)),
        endings : endings,
        closing : closing,
        modifier : metadata,
        point_to_skip : skip_node,
    };
    match builder.generate_dijkstra(&serving_model.graph, &rod_controller) {
        Ok(x) => x,
        Err(e) => {warn!("An error has occurred: {}", e); (Vec::new(), Vec::new())}
    }
}

/// Create a rod.
pub fn create_rod(serving_model : &ServingModel, pos : &Location, metadata : &mut Metadata)
    -> Result<AnnotatedPath<Distance>, RoutingError> {
    let edge = match serving_model.get_edge(pos) {Some(x) => x, _ => return Err(RoutingError::NoSuchEdge(pos.clone()))};

    // check whether we have a previous route. In this case, we'd like to mark our previous point as illegal.
    let (starting_node, skip_node) = match metadata.original_route {
        Some(ref mut route) => {
            let edge_nodes = [edge.edge.from_node, edge.edge.to_node];
            let edge_node_ref : &[NodeID] = &edge_nodes as &[NodeID];
            let occurrences = route.get_first_occuring(edge_node_ref);
            let res = match occurrences.into_iter().next() {
                None => return Err(RoutingError::NotIntersectingRoute(edge.edge.from_node, edge.edge.to_node)),
                Some(x) => (edge.edge.from_node + edge.edge.to_node - x, x),
            };
            if ! route.truncate(res.1) {
                return Err(RoutingError::NotIntersectingRoute(edge.edge.from_node, edge.edge.to_node));
            }
            (res.0, Some(res.1))
        },
        None => (edge.edge.from_node, None),
    };

    // create the tree.
    let (actions, endings) = if let Some(ref route) = metadata.original_route {
        create_field_poison(serving_model, starting_node, VecMap::new(), &*metadata, false, skip_node, route)
    } else {
        create_field_no_poisoning(serving_model, starting_node, VecMap::new(), &*metadata, false, skip_node)
    };

    // Select the best path from the tree.
    let path_length = metadata.original_route.as_ref().map(|r| super::util::path_length(r, &serving_model.graph).to_f64()).unwrap_or(0.0);
    let mut selector = Selector::new_default_rng();
    for &ending in &endings {
        let major = &actions[ending].major;
        if major.actual_length + path_length < metadata.requested_length.to_f64() / 2.0
        {continue;}
        selector.update(major.minor_value * (-major.illegal_node_hits * 5.0 + EVENT_IMPORTANCE *  major.potential_track / major.actual_length).exp(), ending);
    }

    selector.decompose().map(|last| {

        //info!("Chosen rod : {:#?}", actions[last]);
        into_annotated_nodes(&actions, last)
    }).ok_or(RoutingError::NothingSelected)

}

/// Close a rod.
pub fn close_rod(serving_model : &ServingModel, pos : &Location, metadata : &mut Metadata, path : &AnnotatedPath<Distance>)
    -> Result<(Path, Km), RoutingError> {
    // Find the starting point of our rod.
    let edge = match serving_model.get_edge(pos) {Some(x) => x, None => return Err(RoutingError::NoSuchEdge(pos.clone()))};
    let starting_node = edge.edge.from_node;

    // Retrieve the original route, to append at the end.
    let original_route = metadata.original_route.clone().unwrap_or_else(|| Path::new(Vec::new()));
    metadata.requested_length = metadata.requested_length - (original_route.get_elements(&serving_model.graph).1)
        .into_iter().map(|x| x.dist).fold(Km::from_f64(0.0), |x, y| x + y);

    // Create the possible ending points
    let map = path.as_map();
    let map : VecMap<_> = map.into_iter().map(|(n, c)| (n, c.clone())).collect();

    // Get the tree.
    let (actions, endings) = create_field_poison(serving_model, starting_node, map, &*metadata, true , None,
        &path.get_path_filtered(|distance|
            distance.actual_length >= metadata.requested_length.to_f64() * 0.125
            && distance.actual_length <= metadata.requested_length.to_f64() * 0.375));

    // Find the best path in the tree.
    let mut selector = Selector::new_default_rng();
    let mut selector_large = Selector::new_default_rng();
    let map = path.as_map();
    let mut count = 0;
    for &ending in &endings {
        if ending == 0 {
            continue;
        }
        let node = actions[ending].node_handle;
        let distance = &actions[ending].major;
        let total_distance = distance.actual_length + map[node as usize].actual_length;
        let total_weight = distance.minor_value + map[node as usize].minor_value;
        let events = distance.potential_track + map[node as usize].potential_track;
        trace!("Totals of {} : abs({}) rel({}) ({:?}) ", ending, total_distance, total_weight, distance);
        count += 1;
        if total_distance <= metadata.requested_length.to_f64() {
            selector.update((total_distance + EVENT_IMPORTANCE * events / total_distance).exp(), ending);
        } else {
            selector_large.update((-total_distance + EVENT_IMPORTANCE * events / total_distance).exp(), ending);
        }
    }

    info!("Routes selected : {} / {}", count, endings.len());
    let longest_index = selector.decompose().or_else(|| selector_large.decompose());

    longest_index.map(|longest_index| {
        // Compute the actual length of the path.
        let true_length = actions[longest_index].major.actual_length + map[actions[longest_index].node_handle as usize].actual_length;
        debug!("Length: {}", true_length);
        // Simplify and join the path.
        let final_path = path.as_path().join(into_annotated_nodes(&actions, longest_index).as_path());
        let path = original_route.append(final_path);
        (path, Km::from_f64(true_length))
    }).ok_or(RoutingError::NothingSelected)
}
