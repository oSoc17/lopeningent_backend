//! Structure used for putting data on a heap, for Dijkstra purposes.
//! It also inverts the comparison operator, which is useful since the
//! Binary Heap data structure in Rust yields all data in high-to-low order.

use std::cmp::Ordering;
use graph::ordering::Majorising;
use graph::NodeID;
use graph::dijkstra::DijkstraControl;
use num::traits::WrappingSub;

/// HeapData struct

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct HeapData
{
    pub hint : u64,
    pub index : usize,
    pub node: NodeID,
}

impl HeapData
{
    /// Create a new HeapData struct
    pub fn new<C : DijkstraControl>(index : usize, node: NodeID, major : &C::M, control : &C) -> HeapData {
        HeapData {
            index : index,
            node: node,
            hint : 0.wrapping_sub(&control.hint(major)),
        }
    }
}