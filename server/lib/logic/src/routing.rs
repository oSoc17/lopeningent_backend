use graph::{Path, AnnotatedPath};
use data::Conversion;

use graph::dijkstra::DijkstraBuilder;
use graph::dijkstra::DijkstraControl;
use graph::dijkstra::into_annotated_nodes;
use graph::Majorising;

use database::{Node, Edge, Poi, Tags};
use annotated::{PoiNode, AnnotatedEdge};

use newtypes;
use newtypes::{Location, Located};
use newtypes::ToF64;
use newtypes::Km;
use std::f64;

use std::io;
use std::io::Write;

use vec_map::VecMap;

use na;

use util;
use util::selectors::Selector;

use consts::EARTH_RADIUS;

#[derive(PartialEq, Debug, Clone)]
pub struct Distance(f64, f64, f64);

impl Majorising for Distance {
    fn majorises(&self, other : &Self) -> bool {
        (self.0, self.1).majorises(&(other.0, other.1))
    }
}

pub struct Metadata {
    pub requested_length : Km,
}

trait Poisoned {
    fn poison(&self, location: &Location) -> f64;
}

impl Poisoned for () {
    fn poison(&self, location: &Location) -> f64 {
        1.0
    }
}

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

impl Poisoned for PoisonMiddle {
    fn poison(&self, location: &Location) -> f64 {
        let value = self.size - util::distance::distance_lon_lat(location, &self.midpoint, Km::from_f64(EARTH_RADIUS)).to_f64();
        if value > 0.0 {
            (value / self.size * (200.0f64).ln()).exp()
        } else {
            1.0
        }
    }
}

pub struct PoisonLine {
    start : na::Vector3<f64>,
    end : na::Vector3<f64>,
    radius : f64,
    size : f64,
}

impl PoisonLine {
    pub fn new(start : &Location, end : &Location) -> PoisonLine {
        PoisonLine {
            start : start.into_3d(),
            end : end.into_3d(),
            radius : EARTH_RADIUS,
            size : util::distance::distance_lon_lat(start, end, Km::from_f64(EARTH_RADIUS)).to_f64(),
        }
    }
}

impl Poisoned for PoisonLine {
    fn poison(&self, location: &Location) -> f64 {
        let pos = location.into_3d();
        let value_vec = (pos - self.start).cross(&(self.end - self.start));
        let value = self.size - self.radius * value_vec.dot(&value_vec).sqrt().sqrt();
        if value > 0.0 {
            (value / self.size * (500.0f64).ln()).exp()
        } else {
            1.0
        }
    }
}

struct RodController<P : Poisoned> {
    max_length : f64,
    poisoner : P,
    endings : VecMap<Distance>,
    closing : bool,
}

impl<P : Poisoned> RodController<P> {
    fn enjoyment(&self, tag : &Tags) -> f64 {
        1.0
    }

    fn annotate(&self, edge : &AnnotatedEdge) -> Distance {
        let t = edge.dist.to_f64();
        let p = self.poisoner.poison(&edge.average);
        let e = self.enjoyment(&edge.edge.tags);
        Distance(t * e * (1.0 + p / 5.0),  t * p * e, t)
    }
}

impl<P : Poisoned> DijkstraControl for RodController<P> {
    type V = PoiNode;
    type E = AnnotatedEdge;
    type M = Distance;
    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M {
        let added = self.annotate(e);
        Distance(m.0 + added.0, m.1 + added.1, m.2 + added.2)
    }
    fn filter(&self, m : &Self::M) -> bool {
        m.2 < self.max_length
    }
    fn hint(&self, m : &Self::M) -> u64 {
        (m.2 * 1000000.0) as u64
    }
    fn is_ending(&self, v : &Self::V, m : &Self::M) -> bool {
        self.endings.get(v.node.nid).is_some()
    }
    fn yield_on_empty(&self) -> bool {
        !self.closing
    }
}

pub fn create_rod<'a>(conversion : &Conversion<'a>, pos : &Location, metadata : &Metadata) -> Option<AnnotatedPath<Distance>> {
    let edge = match conversion.get_edge(pos) {Some(x) => x, _ => return None};
    let starting_node = edge.edge.from_node;
    let builder = DijkstraBuilder::new(starting_node, Distance(0.0, 0.0, 0.0));
    let rod_controller = RodController{
        max_length : metadata.requested_length.to_f64(),
        poisoner : (),
        endings : VecMap::new(),
        closing : false,
    };
    let (actions, endings) = builder.generate_dijkstra(conversion.graph, &rod_controller);

    let mut selector = Selector::new_default_rng();
    for &ending in &endings {
        if actions[ending].major.2 < metadata.requested_length.to_f64() / 2.0
        {continue;}
        selector.update(actions[ending].major.1, ending);
    }

    selector.decompose().map(|last| {

        writeln!(io::stderr(), "Chosen rod : {:#?}", actions[last]);
        into_annotated_nodes(&actions, last)
    })

}

pub fn close_rod<'a>(conversion : &Conversion<'a>, pos : &Location, metadata : &Metadata, path : AnnotatedPath<Distance>) -> Option<(Path, Km)> {
    let map = path.into_map();
    let map : VecMap<_> = map.into_iter().map(|(n, c)| (n, c.clone())).collect();
    let edge = match conversion.get_edge(pos) {Some(x) => x, None => return None};
    let starting_node = edge.edge.from_node;
    let builder = DijkstraBuilder::new(starting_node, Distance(0.0, 0.0, 0.0));
    let rod_controller = RodController {
        max_length : metadata.requested_length.to_f64(),
        poisoner : PoisonLine::new(&conversion.graph.get(path.first().0).unwrap().located(), &conversion.graph.get(path.last().0).unwrap().located()),
        endings : map,
        closing : true,
    };
    let (actions, endings) = builder.generate_dijkstra(conversion.graph, &rod_controller);

    let mut selector = Selector::new_default_rng();
    let map = rod_controller.endings;
    let mut count = 0;
    for &ending in &endings {
        if ending == 0 {
            continue;
        }
        let node = actions[ending].node_handle;
        let distance = &actions[ending].major;
        let total_distance = distance.2 + map[node].2;
        let total_weight = distance.1 + map[node].1;
        if total_distance >= metadata.requested_length.to_f64() {
            continue;
        }
        if total_distance <= metadata.requested_length.to_f64() * 0.8 {
            continue;
        }
        writeln!(io::stderr(), "Totals of {} : abs({}) rel({}) ({:?})", ending, total_distance, total_weight, distance);
        count += 1;
        selector.update((total_distance).exp(), ending);
    }

    writeln!(io::stderr(), "Routes selected : {} / {}", count, endings.len());
    let longest_index = selector.decompose();
    longest_index.map(|longest_index| {
        let prev_node = &actions[actions[longest_index].previous_index].node_handle;
        writeln!(io::stderr(), "longest_index : {} {} {}", longest_index, actions[longest_index].previous_index, prev_node);
        writeln!(io::stderr(), "contains : {}", map.get(*prev_node).is_some());
        writeln!(io::stderr(), "disabled : {}", actions[actions[longest_index].previous_index].disabled);


        let true_length = actions[longest_index].major.2 + map[actions[longest_index].node_handle].2;
        writeln!(io::stderr(), "True length: {}", true_length);
        (path.into_path().join(into_annotated_nodes(&actions, longest_index).into_path()), Km::from_f64(true_length))
    })
}
