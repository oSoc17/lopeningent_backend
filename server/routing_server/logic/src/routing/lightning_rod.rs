use graph::{Path, AnnotatedPath};
use data::Conversion;

use graph::dijkstra::DijkstraBuilder;
use graph::dijkstra::{DijkstraControl, Ending, SingleAction};
use graph::dijkstra::into_annotated_nodes;
use graph::Majorising;
use graph::NodeID;

use database::{Tags, TagConverter};
use database::TagModifier;
use annotated::{PoiNode, AnnotatedEdge};


use newtypes::{Location, Located};
use newtypes::ToF64;
use newtypes::Km;

use std::f64;
use std::io;
use std::io::Write;
use std::collections::HashSet;

use std::sync::atomic::Ordering;

use vec_map::VecMap;

use na;

use util;
use util::selectors::Selector;

use consts::*;
use super::util::Metadata;


#[derive(PartialEq, Debug, Clone, Default)]
pub struct Distance{
    pub major_value : f64,
    pub minor_value : f64,
    pub actual_length : f64,
    pub illegal_node_hits : f64,
    pub node_potential : f64,
    pub potential_track : f64,
}

impl Distance {
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

    pub fn def() -> Distance {
        let mut res = Self::default();
        res.node_potential = 1.0;
        res
    }

    pub fn get_length(&self) -> f64 {
        self.actual_length
    }

    pub fn get_potential(&self) -> f64 {
        self.major_value
    }

    fn into_majorising(&self) -> (f64, f64, f64) {
        (self.major_value, self.minor_value, self.illegal_node_hits)//, self.node_potential)
    }
}


impl Majorising for Distance {
    fn majorises(&self, other : &Self) -> bool {
        self.into_majorising().majorises(&other.into_majorising())
    }
}

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

pub struct PoisonLine {
    start : na::Vector3<f64>,
    end : na::Vector3<f64>,
    radius : f64,
    size : f64,
    maximum_ln : f64,
}

impl PoisonLine {
    pub fn new(start : &Location, end : &Location, factor : f64, maximum : f64) -> PoisonLine {
        PoisonLine {
            start : start.into_3d(),
            end : end.into_3d(),
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
        let random_factor = (edge.hits.load(Ordering::Relaxed) as f64 + 20.0);
        let random_factor = random_factor * random_factor * util::selectors::get_random(0.1, 1.0);
        let res = Distance::new((t * n_p * p_l * random_factor,  t * n_p * p_s * random_factor, t , hit_illegal_node, n_p, -e));
        //let _ = writeln!(io::stderr(), "{} -> {} : {:?}", edge.edge.from_node, edge.edge.to_node, res);
        res
    }
}

impl<'a, P : Poisoned, TM : TagModifier + 'a> DijkstraControl for RodController<'a, P, TM> {
    type V = PoiNode;
    type E = AnnotatedEdge;
    type M = Distance;
    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M {
        let added = self.annotate(e, m.node_potential);
        let res = Distance {
            major_value : m.major_value + added.major_value,
            minor_value : m.minor_value + added.minor_value,
            actual_length : m.actual_length + added.actual_length,
            illegal_node_hits : m.illegal_node_hits + added.illegal_node_hits,
            node_potential : added.node_potential,
            potential_track : m.potential_track + added.potential_track,
        };
        res
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

pub fn create_field_no_poisoning(conversion : &Conversion, starting_node : NodeID, endings : VecMap<Distance>, metadata : &Metadata, closing : bool, skip_node : Option<NodeID>)
    -> (Vec<SingleAction<Distance>>, Vec<usize>) {
    let builder = DijkstraBuilder::new(starting_node, Distance::def());
    let rod_controller = RodController{
        max_length : metadata.requested_length.to_f64(),
        poisoner_large : (),
        poisoner_small : (),
        endings : endings,
        closing : closing,
        modifier : metadata,
        point_to_skip : skip_node,
    };
    builder.generate_dijkstra(&conversion.graph, &rod_controller)
}

pub fn create_field_poison(conversion : &Conversion, starting_node : NodeID, endings : VecMap<Distance>, metadata : &Metadata, closing : bool, skip_node : Option<NodeID>, poison_path : &Path)
    -> (Vec<SingleAction<Distance>>, Vec<usize>) {
    let builder = DijkstraBuilder::new(starting_node, Distance::def());
    let large_random = util::selectors::get_random(CONFIG.min, CONFIG.max);
    let small_random = large_random - CONFIG.increase;//util::selectors::get_random(0.3, 0.5);

    let location_from = &conversion.graph.get(poison_path.first()).unwrap().located();
    let location_to = &conversion.graph.get(poison_path.last()).unwrap().located();
    let min_distance = util::distance::distance_lon_lat(location_from, location_to, Km::from_f64(EARTH_RADIUS));
    let req_length = if closing {
        metadata.requested_length
    } else {
        metadata.requested_length - super::util::path_length(&poison_path, &conversion.graph)
    };

    let min_distance = if req_length.to_f64() < min_distance.to_f64() * 1.2 {
        min_distance * 1.2
    } else {
        req_length
    };

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
    builder.generate_dijkstra(&conversion.graph, &rod_controller)
}

pub fn create_rod(conversion : &Conversion, pos : &Location, metadata : &mut Metadata) -> Option<AnnotatedPath<Distance>> {
    let edge = match conversion.get_edge(pos) {Some(x) => x, _ => return None};
    let (starting_node, skip_node) = match metadata.original_route {
        Some(ref mut route) => {
            let q : HashSet<_> = route.get_indices().iter().cloned().collect();
            let res = if let Some(_) = q.get(&edge.edge.to_node) {
                (edge.edge.from_node, edge.edge.to_node)
            } else {
                (edge.edge.to_node, edge.edge.from_node)
            };
            if ! route.truncate(res.1) {
                return None;
            }
            (res.0, Some(res.1))
        },
        None => (edge.edge.from_node, None),
    };
    let (actions, endings) = if let Some(ref route) = metadata.original_route {
        create_field_poison(conversion, starting_node, VecMap::new(), &*metadata, false, skip_node, route)
    } else {
        create_field_no_poisoning(conversion, starting_node, VecMap::new(), &*metadata, false, skip_node)
    };
    let path_length = metadata.original_route.as_ref().map(|r| super::util::path_length(r, &conversion.graph).to_f64()).unwrap_or(0.0);
    let mut selector = Selector::new_default_rng();
    for &ending in &endings {
        let major = &actions[ending].major;
        if major.actual_length + path_length < metadata.requested_length.to_f64() / 2.0
        {continue;}
        selector.update(major.minor_value * (-major.illegal_node_hits * 5.0 + EVENT_IMPORTANCE *  major.potential_track / major.actual_length).exp(), ending);
    }

    selector.decompose().map(|last| {

        //let _ = writeln!(io::stderr(), "Chosen rod : {:#?}", actions[last]);
        into_annotated_nodes(&actions, last)
    })

}

use std::time;

pub fn close_rod(conversion : &Conversion, pos : &Location, metadata : &mut Metadata, path : AnnotatedPath<Distance>) -> Option<(Path, Km)> {
    let now = time::Instant::now();
    let edge = match conversion.get_edge(pos) {Some(x) => x, None => return None};
    let starting_node = edge.edge.from_node;

    let original_route = metadata.original_route.clone().unwrap_or_else(|| Path::new(Vec::new()));
    metadata.requested_length = metadata.requested_length - (original_route.get_elements(&conversion.graph).1)
        .into_iter().map(|x| x.dist).fold(Km::from_f64(0.0), |x, y| x + y);

    let map = path.as_map();
    let map : VecMap<_> = map.into_iter().map(|(n, c)| (n, c.clone())).collect();

    let (actions, endings) = create_field_poison(conversion, starting_node, map, &*metadata, true , None, &path.as_path());

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
        let _ = writeln!(io::stderr(), "Totals of {} : abs({}) rel({}) ({:?}) ", ending, total_distance, total_weight, distance);
        count += 1;
        if total_distance <= metadata.requested_length.to_f64() {
            selector.update((total_distance + EVENT_IMPORTANCE * events / total_distance).exp(), ending);
        } else {
            selector_large.update((-total_distance + EVENT_IMPORTANCE * events / total_distance).exp(), ending);
        }
    }

    let duration = time::Instant::now() - now;

    let _ = writeln!(io::stderr(), "Routes selected : {} / {}", count, endings.len());
    let longest_index = selector.decompose().or(selector_large.decompose());

    longest_index.map(|longest_index| {
        let prev_node = &actions[actions[longest_index].previous_index].node_handle;
        let true_length = actions[longest_index].major.actual_length + map[actions[longest_index].node_handle as usize].actual_length;
        let _ = writeln!(io::stderr(), "Length: {}", true_length);
        let final_path = path.as_path().join(into_annotated_nodes(&actions, longest_index).as_path());
        let path = original_route.append(final_path);
        (path, Km::from_f64(true_length))
    })
}
