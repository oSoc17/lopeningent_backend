use std::sync::Arc;
use database::Node;
use database::Edge;
use database::Poi;
use newtypes::{Located, Location, Km};
use na;

use graph::Graph;

pub type ApplicationGraph = Graph<PoiNode, AnnotatedEdge>;

#[derive(Debug)]
pub struct PoiNode {
    pub node : Node,
    pub poi : Option<Vec<Arc<Poi>>>
}

impl Located for PoiNode {
    fn located(&self) -> Location {
        self.node.located()
    }
}

#[derive(Debug)]
pub struct AnnotatedEdge {
    pub edge : Edge,
    pub dist : Km,
    pub average : na::Vector3<f64>
}
