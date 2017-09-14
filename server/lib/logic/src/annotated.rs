use std::sync::Arc;
use database::Node;
use database::Edge;
use database::Poi;
use newtypes::{Located, Location, Km};

#[derive(Debug)]
pub struct PoiNode {
    pub node : Node,
    pub poi : Option<Arc<Vec<Poi>>>
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
    pub average : Location,
}
