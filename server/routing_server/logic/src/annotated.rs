/// This module contains annotated versions of the Node and Edge types.

use std::sync::Arc;
use database::Node;
use database::Edge;
use database::Poi;
use newtypes::{Located, Location, Km};
use na;
use std::sync::atomic::AtomicUsize;


use graph::Graph;

/// The graph we're actually working on.
///
/// Importing three different structures is too cumbersome.
pub type ApplicationGraph = Graph<PoiNode, AnnotatedEdge>;

/// Annotated node
#[derive(Debug)]
pub struct PoiNode {
    /// Node
    pub node : Node,
    /// List of all poi's on this point.
    ///
    /// Wrapped into an optional because... Because of reasons.
    pub poi : Option<Vec<Arc<Poi>>>
}

impl Located for PoiNode {
    fn located(&self) -> Location {
        self.node.located()
    }
}

/// Annotated edge.
#[derive(Debug)]
pub struct AnnotatedEdge {
    /// Edge
    pub edge : Edge,
    /// Length of the edge.
    pub dist : Km,
    /// Position on a 3D unit sphere.
    pub average : na::Vector3<f64>,
    /// How often a route passed this edge.
    pub hits : AtomicUsize,
}
