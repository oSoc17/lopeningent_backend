//! Utility to create simple graphs for testing.

use Graph;

use NodeID;
use error::Error;

/// Create a simple checkerboard graph.
pub fn create_testgraph<V, E, FV : FnMut(usize, usize) -> V, FE : FnMut(NodeID, NodeID) -> E>
    (w : usize, h : usize, fv : FV, fe : FE) -> Result<Graph<V, E>, Error> {
    let mut fv = fv;
    let mut fe = fe;
    let node_iterator = (0..w).flat_map(|x|
            {
                (0..h).map(|y| (x as NodeID * h as NodeID + y as NodeID, (&mut fv)(x, y))).collect::<Vec<_>>()
            });
    let edge_iterator = (0..w).flat_map(|x| (0..h).flat_map(|y|
        {
            if x > 0     {Some((x - 1, y))} else {None}.into_iter().chain(
            if x < w - 1 {Some((x + 1, y))} else {None}.into_iter().chain(
            if y > 0     {Some((x, y - 1))} else {None}.into_iter().chain(
            if y < h - 1 {Some((x, y + 1))} else {None}.into_iter())))
        }.map(|(dx, dy)| (x as NodeID * h as NodeID + y as NodeID, dx as NodeID * h as NodeID + dy as NodeID))
        .map(|(from, to)| (from, (&mut fe)(from, to), to)).collect::<Vec<_>>()).collect::<Vec<_>>());

    Graph::new(node_iterator, edge_iterator)
}
