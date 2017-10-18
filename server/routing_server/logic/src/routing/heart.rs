/// Experimental algorithm for routing. Do not use yet (can't even compile it.)

use PoiNode;
use AnnotatedEdge;
use graph::dijkstra::{DijkstraBuilder, DijkstraControl, SingleAction, Ending};
use graph::NodeID;
use na::Vector3;
use vec_map::VecMap;


struct Distance {
    actual_length : f64,
    swirl_length : f64,
}

impl Distance {
    fn into_majorising(&self) -> f64 {
        self.swirl_length;
    }
}

impl Majorising for Distance {
    fn majorises(&self, other : &Self) -> bool {
        self.into_majorising().majorises(&other.into_majorising())
    }
}


struct Swirler {
    point : Vector3<f64>,
}

pub trait Mapper {

}

impl Mapper for () {

}

impl Mapper for VecMap<Distance> {

}

struct HalfController<M : Mapper> {
    swirler : Swirler,
    mapper : M,

}

impl<M> DijkstraControl for HalfController<M> {
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
        self.closing
    }
}
