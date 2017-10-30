/// Module for error handling

use std::error::Error;
use graph::NodeID;
use newtypes::Location;

/// Error type
#[derive(Debug)]
pub enum RoutingError {
    /// Something else
    Other(Box<Error>),
    /// If the position is too far away from the map.
    NoSuchEdge(Location),
    /// If the return route is way off.
    NotIntersectingRoute(NodeID, NodeID),
    /// If no path has been found due to other reasons.
    NothingSelected,
    /// If the path hasn't been computed yet.
    Empty,
}

impl RoutingError {
    /// Create a general exception.
    fn general<E : 'static + Error>(err : E) -> RoutingError {
        RoutingError::Other(Box::new(err))
    }
}
