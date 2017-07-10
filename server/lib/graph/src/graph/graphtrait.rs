//! Graph trait
//!
//!

use std::iter::FromIterator;
use std::ops::Add;
use std::rc::Rc;
use std::cell::RefCell;

use std::collections::BinaryHeap;
use std::collections::HashMap;

use num::Zero;

use graph::HeapData;
use graph::iter;
use graph::dijkstra::DijkstraGenerator;

/// Graph Trait.
///
/// This trait contains a few interesting functions that every graph should implement,
/// and that can be used to build upon, to create higher order algorithms.
pub trait GraphTrait : Sized {
    /// Node type
    type V;
    /// Edge type
    type E;

    /// Retrieve a single node given the index.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// use graph::GraphTrait;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// assert_eq!(graph.get(0), Some(&"A"));
    /// assert_eq!(graph.get(1), None);
    /// ```
    fn get(&self, index : usize) -> Option<&Self::V>;

    /// Retrieve all connections to a node
    ///
    /// This function returns an iterator, iterating over all edges that are connected
    /// to this node, in a (node\_id, edge\_data) fashion.
    ///
    /// No guarantees need to be made about the order in which the nodes or edges appear.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// use graph::GraphTrait;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// let mut connections = graph.get_conn_idval(0).expect("This does not happen");
    ///
    /// assert_eq!(connections.next(), Some((5, &"Edge from A to B")));
    /// assert_eq!(connections.next(), None);
    /// ```
    fn get_conn_idval<'a>(&'a self, index : usize) -> Option<iter::ConnIdVal<'a, Self::E>>;

    /// create a new Dijkstra's iterator.
    ///
    /// See also [the dijkstra module](dijkstra/index.html).
    fn gen_limited_dijkstra_vec<'a, O, MF, FF> (&'a self, start_nodes : &[usize], map_fn : MF , filter_fn : FF)
        -> DijkstraGenerator<'a, Self::V, Self::E, O, MF, FF, Self>
        where O : Eq + Ord + Zero,
              for<'p, 'q> &'p O : Add<&'q O, Output = O>,
              MF : Fn(&Self::E) -> O,
              FF : Fn(usize, &O) -> bool,
    {
        let heap = BinaryHeap::from_iter(start_nodes.iter().map(|&node| HeapData::new(node, O::zero())));
        DijkstraGenerator::new(
            self,
            heap,
            Rc::new(RefCell::new(HashMap::new())),
            map_fn,
            filter_fn,
        )
    }
}
