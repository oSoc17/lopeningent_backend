//! Structure used for putting data on a heap, for Dijkstra purposes.
//! It also inverts the comparison operator, which is useful since the
//! Binary Heap data structure in Rust yields all data in high-to-low order.

use std::ops::Add;
use std::cmp::Ordering;

/// HeapData struct
///
/// # Examples
/// ```
/// use graph::HeapData;
/// let data = HeapData::new(0, 0);
/// let update = data.update(&7, 1);
/// let update = update.update(&12, 2);
/// assert_eq!(update, HeapData{val : 19, node : 2, from : Some(1)});
/// ```
#[derive(Eq, PartialEq, Debug)]
pub struct HeapData<O>
    where O: PartialEq + Eq + PartialOrd + Ord,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
{
    /// Value of the data, on which the heap is sorted
    pub val: O,
    /// Node
    pub node: usize,
    /// Previous node
    ///
    /// The combination of node and previous node can be inserted into a linked hashmap.
    pub from: Option<usize>,
}

impl<O> HeapData<O>
    where O: PartialEq + Eq + PartialOrd + Ord,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
{
    /// Create a new HeapData struct
    pub fn new(node: usize, zero : O) -> HeapData<O> {
        HeapData {
            val: zero,
            node: node,
            from: None,
        }
    }
    /// Update the HeapData struct, as if the new node can be connected to the old node
    /// at a distance.
    pub fn update(&self, distance: &O, newnode: usize) -> HeapData<O> {
        HeapData {
            val: distance + &self.val,
            node: newnode,
            from: Some(self.node),
        }
    }
}


impl<O> Ord for HeapData<O>
    where O: PartialEq + Eq + PartialOrd + Ord,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl<O> PartialOrd for HeapData<O>
    where O: PartialEq + Eq + PartialOrd + Ord,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
