//! Structure used for putting data on a heap, for Dijkstra purposes.
//! It also inverts the comparison operator, which is useful since the
//! Binary Heap data structure in Rust yields all data in high-to-low order.



use graph::NodeID;
use graph::dijkstra::DijkstraControl;
use num::traits::WrappingSub;

/// HeapData struct

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct HeapData
{
    /// Hint for speedup using heap.
    pub hint : u64,
    /// SingleAction vector index.
    pub index : usize,
    /// representative node.
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
