//! Main class for performing Dijkstra's algorithm.
//!
//! The idea of this class is to process a graph and yield all its elements in
//! order of distance.
//!
//! # Examples
//! ```
//! use graph::Graph;
//! //The actual way to create such an iterator is through the 'GraphTrait' trait;
//! use graph::GraphTrait;
//! let graph = Graph::new(
//!             vec![(0, "A"), (5, "B")],
//!             vec![(0, "Edge from A to B", 5)]
//!     ).expect("This does not happen");
//!
//! let mut iter = graph.gen_limited_dijkstra_vec(&vec![0], |edge| edge.len(), |_, _| true);
//!
//! // Node 0 is at distance 0 (obviously)
//! assert_eq!(iter.next(), Some((0, 0)));
//! // Node 5 is at distance "Edge from A to B".len() = 16
//! assert_eq!(iter.next(), Some((5, 16)));
//! ```
//!
//! Additionally the route to the source can be queried using the "Visited" field:
//!
//! ```
//! # use graph::Graph;
//! # use graph::GraphTrait;
//! # let graph = Graph::new(
//! #             vec![(0, "A"), (5, "B"), (3, "C")],
//! #             vec![(0, "Edge from A to B", 5)]
//! #     ).expect("This does not happen");
//!
//! # let mut iter = graph.gen_limited_dijkstra_vec(&vec![0], |edge| edge.len(), |_, _| true);
//! # iter.next();
//! # iter.next();
//! // Lock the structure on the stack
//! let visited = iter.visited();
//! // Borrow the structure
//! let visited = visited.borrow();
//!
//! // The node 5 is visited via 0, so the shortest path from 0 to 5 is through 0.
//! assert_eq!(visited.get(&5), Some(&Some(0)));
//! // 0 is the origin, so it has no previous node.
//! assert_eq!(visited.get(&0), Some(&None));
//! // 3 is not visited.
//! assert_eq!(visited.get(&3), None);
//! ```


use std::collections::BinaryHeap;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;

use num::Zero;

use graph::HeapData;
use graph::graphtrait::GraphTrait;

/// Shared linked hashmap for a path.
///
/// This type is a representation of a shared linked hashmap: The hashmap contains links:
///
/// `Visited == ID -> Option<ID>`
///
/// Consider, for example, the hashmap with the contents:
/// ```text
///     0 -> None,
///     1 -> Some(0),
///     2 -> Some(0),
///     3 -> Some(1),
///     4 -> Some(1),
/// ```
/// The following paths can be found in this map:
/// ```text
///     [0]
///     [1, 0]
///     [2, 0]
///     [3, 1, 0]
///     [4, 1, 0]
/// ```

pub type Visited = Rc<RefCell<HashMap<usize, Option<usize>>>>;

/// Iterator for generating all edges and nodes in a graph, in a Dijkstra's fashion.
///
/// See [the module level documentation](index.html) for an example.
pub struct DijkstraGenerator<'a, V : 'a, E : 'a, O, MF, FF, G>
    where O : Eq + Ord + Zero,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
          MF : Fn(&E) -> O,
          FF : Fn(usize, &O) -> bool,
           G : GraphTrait<V=V, E=E> + 'a
{
    graph : &'a G,
    heap : BinaryHeap<HeapData<O>>,
    visited : Visited,
    mf : MF,
    ff : FF,
}

impl<'a, V : 'a, E : 'a, O, MF, FF, G>
    DijkstraGenerator<'a, V, E, O, MF, FF, G>
    where O : Eq + Ord + Zero,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
          MF : Fn(&E) -> O,
          FF : Fn(usize, &O) -> bool,
           G : GraphTrait<V=V, E=E> + 'a
{
    /// Create a new iterator
    pub fn new(graph : &'a G, heap : BinaryHeap<HeapData<O>>, visited : Visited, mf : MF, ff : FF) -> Self {
        DijkstraGenerator {
            graph : graph,
            heap : heap,
            visited : visited,
            mf : mf,
            ff : ff,
        }
    }

    /// Return a linked hashmap containing the shortest paths.
    pub fn visited(&self) -> Visited {
        self.visited.clone()
    }
}


impl<'a, V : 'a, E : 'a, O, MF, FF, G> Iterator for DijkstraGenerator<'a, V, E, O, MF, FF, G>
    where O : Eq + Ord + Zero,
          for<'p, 'q> &'p O : Add<&'q O, Output = O>,
          MF : Fn(&E) -> O,
          FF : Fn(usize, &O) -> bool,
           G : GraphTrait<V=V, E=E> + 'a
{
    type Item = (usize, O);
    fn next(&mut self) -> Option<Self::Item> {
        let mut visited = self.visited.borrow_mut();
        loop {
            match self.heap.pop() {
                None => return None,
                Some(hdata) => {
                    if let Some(_) = visited.get(&hdata.node)
                        {continue;}
                    visited.insert(hdata.node, hdata.from);
                    if let Some(connidvaliter) = self.graph.get_conn_idval(hdata.node) {
                        for (next_id, edge) in connidvaliter {
                            let next_length = (self.mf)(edge);
                            let next_hdata = hdata.update(&next_length, next_id);
                            if (self.ff)(next_hdata.node, &next_hdata.val) {
                                self.heap.push(next_hdata);
                            }
                        }
                    }
                    return Some((hdata.node, hdata.val))
                }
            }
        }
    }
}
