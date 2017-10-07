use graph::{Path, AnnotatedPath};
use data::Conversion;

use graph::dijkstra::DijkstraBuilder;
use graph::dijkstra::{DijkstraControl, Ending};
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
}

impl Distance {
    pub fn new(values : (f64, f64, f64, f64, f64)) -> Distance {
        Distance {
            major_value : values.0,
            minor_value : values.1,
            actual_length : values.2,
            illegal_node_hits : values.3,
            node_potential : values.4,
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
        let e = self.enjoyment(&edge.edge.tags);
        if e != 0.0 {
            next_potential *= e.exp() * DILUTE_FAVOURITE;
            let (min, max) = M::tag_bounds();
            if next_potential > max {
                next_potential = max;
            }
            if next_potential < min {
                next_potential = min;
            }
        }
        let n_p = next_potential;
        let random_factor = (edge.hits.load(Ordering::Relaxed) as f64 + 20.0);
        let random_factor = random_factor * random_factor * util::selectors::get_random(0.1, 1.0);
        Distance::new((t * n_p * p_l * random_factor,  t * n_p * p_s * random_factor, t , 0.0, n_p))
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
            illegal_node_hits : m.illegal_node_hits +
            if Some(e.edge.to_node) == self.point_to_skip {1.0} else {0.0},
            node_potential : added.node_potential,
        };
        //println!("{:?}", res);
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
                if m.actual_length + dist.actual_length <= self.max_length
                    && m.actual_length + dist.actual_length > self.max_length * MIN_LENGTH_FACTOR
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
        true
    }
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
            metadata.requested_length = metadata.requested_length - (route.get_elements(&conversion.graph).1)
                .into_iter().map(|x| x.dist).fold(Km::from_f64(0.0), |x, y| x + y);
            (res.0, Some(res.1))
        },
        None => (edge.edge.from_node, None),
    };
    let builder = DijkstraBuilder::new(starting_node, Distance::def());
    let rod_controller = RodController{
        max_length : metadata.requested_length.to_f64(),
        poisoner_large : (),
        poisoner_small : (),
        endings : VecMap::new(),
        closing : false,
        modifier : &*metadata,
        point_to_skip : skip_node,
    };
    let (actions, endings) = builder.generate_dijkstra(&conversion.graph, &rod_controller);

    let mut selector = Selector::new_default_rng();
    for &ending in &endings {
        if actions[ending].major.actual_length < metadata.requested_length.to_f64() / 2.0
        {continue;}
        selector.update(actions[ending].major.minor_value, ending);
    }

    selector.decompose().map(|last| {

        //let _ = writeln!(io::stderr(), "Chosen rod : {:#?}", actions[last]);
        into_annotated_nodes(&actions, last)
    })

}

use std::time;

pub fn close_rod(conversion : &Conversion, pos : &Location, metadata : &Metadata, path : AnnotatedPath<Distance>) -> Option<(Path, Km)> {
    let now = time::Instant::now();
    let map = path.as_map();
    let map : VecMap<_> = map.into_iter().map(|(n, c)| (n, c.clone())).collect();
    let edge = match conversion.get_edge(pos) {Some(x) => x, None => return None};
    let starting_node = edge.edge.from_node;
    let builder = DijkstraBuilder::new(starting_node, Distance::def());
    let large_random = util::selectors::get_random(CONFIG.min, CONFIG.max);
    let small_random = large_random - CONFIG.increase;//util::selectors::get_random(0.3, 0.5);
    let rod_controller = RodController {
        max_length : metadata.requested_length.to_f64(),
        poisoner_large : PoisonLine::new(&conversion.graph.get(path.first().0).unwrap().located(), &conversion.graph.get(path.last().0).unwrap().located(),
        large_random, util::selectors::get_random(CONFIG.min_lin, CONFIG.max_lin)),
        poisoner_small : PoisonLine::new(&conversion.graph.get(path.first().0).unwrap().located(), &conversion.graph.get(path.last().0).unwrap().located(),
        small_random, util::selectors::get_random(CONFIG.min_lin, CONFIG.max_lin)),
        endings : map,
        closing : true,
        modifier : metadata,
        point_to_skip : None,
    };
    let (actions, endings) = builder.generate_dijkstra(&conversion.graph, &rod_controller);

    let mut selector = Selector::new_default_rng();
    let map = rod_controller.endings;
    let mut count = 0;
    for &ending in &endings {
        if ending == 0 {
            continue;
        }
        let node = actions[ending].node_handle;
        let distance = &actions[ending].major;
        let total_distance = distance.actual_length + map[node as usize].actual_length;
        let total_weight = distance.minor_value + map[node as usize].minor_value;
        //let _ = write!(io::stderr(), "Totals of {} : abs({}) rel({}) ({:?}) ", ending, total_distance, total_weight, distance);
        if total_distance <= metadata.requested_length.to_f64() * MIN_LENGTH_FACTOR {
            //let _ = writeln!(io::stderr(), "Failure!");
            continue;
        }
        //let _ = writeln!(io::stderr(), "Success!");
        count += 1;
        selector.update((total_distance).exp(), ending);
    }

    let duration = time::Instant::now() - now;
    let _ = writeln!(io::stderr(), "{} {} {}.{:09} {}", large_random, small_random, duration.as_secs(), duration.subsec_nanos(), count);

    let _ = writeln!(io::stderr(), "Routes selected : {} / {}", count, endings.len());
    let longest_index = selector.decompose();

    longest_index.map(|longest_index| {
        let prev_node = &actions[actions[longest_index].previous_index].node_handle;
        //let _ = writeln!(io::stderr(), "longest_index : {} {} {}", longest_index, actions[longest_index].previous_index, prev_node);
        //let _ = writeln!(io::stderr(), "contains : {}", map.get(*prev_node as usize).is_some());
        //let _ = writeln!(io::stderr(), "disabled : {}", actions[actions[longest_index].previous_index].disabled);


        let true_length = actions[longest_index].major.actual_length + map[actions[longest_index].node_handle as usize].actual_length;
        let _ = writeln!(io::stderr(), "True length: {}", true_length);
        (path.as_path().join(into_annotated_nodes(&actions, longest_index).as_path()), Km::from_f64(true_length))
    })
}
