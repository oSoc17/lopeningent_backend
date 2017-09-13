//! Graph structure.
//!
//! This data structure forms the basis of the entire crate.


use std::collections::BTreeMap;

use error::Error;

use graph::iter;
use vec_map::VecMap;

pub type NodeID = usize;
pub type EdgeID = usize;

/// Element in a graph
#[derive(Debug)]
pub struct Element<V, E> {
    pub v : V,
    links : BTreeMap<usize, E>,
}

/// Graph structure.
#[derive(Debug)]
pub struct Graph<V, E> {
    data : VecMap<Element<V, E>>,
}

impl<'a, V : 'a, E : 'a> Graph<V, E> {

    /// Create a new graph, with the given vertices and edges.
    ///
    /// Every vertex is of the form `(id : usize, v : V)`, and every edge is of the
    /// form `(from : usize, e : E, to : usize)`, which connect the vertex with index
    /// `from` to the vertex with index `to`.
    ///
    /// Returns: A new graph instance, or an error if an edge originates from a nonexisting vertex.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// ```
    pub fn new<NI, EI>(vertices : NI, edges : EI) -> Result<Graph<V, E>, Error>
        where NI : IntoIterator<Item=(usize, V)>,
              EI : IntoIterator<Item=(usize, E, usize)>
    {
        let mut data = VecMap::new();
        for (id, vertex) in vertices {
            data.insert(id, Element {v : vertex, links : BTreeMap::new()});
        }
        for (id, edge, to) in edges {
            try!(data.get_mut(id).ok_or(Error::MissingID)).links.insert(to, edge);
        }
        Ok(Graph {
            data : data,
        })
    }

    /// Returns whether the graph contains the index.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// assert_eq!(graph.contains(0), true); // The vertex with index 0 has data "A".
    /// assert_eq!(graph.contains(2), false);
    /// ```
    pub fn contains(&self, index : NodeID) -> bool {
        self.data.contains_key(index)
    }

    /// Retrieve a single node given the index.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// assert_eq!(graph.get(0), Some(&"A"));
    /// assert_eq!(graph.get(1), None);
    /// ```
    pub fn get(&self, index : NodeID) -> Option<&V> {
        return self.data.get(index).map(|e| &e.v)
    }

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
    pub fn get_conn_idval<'b>(&'b self, index : NodeID) -> Option<iter::ConnIdVal<'b, E>> {
        self.data.get(index)
            .map(|e| e.links.iter())
            .map(iter::ConnIdVal::new)
    }


    /// Returns all edges connected to a node.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// let mut edges = graph.get_edges(0).expect("This does not happen");
    /// assert_eq!(edges.next(), Some(&"Edge from A to B"));
    /// assert_eq!(edges.next(), None);
    /// ```
    pub fn get_edges(&'a self, index : NodeID) -> Option<iter::IterEdges<'a, E>> {
        self.get_conn_idval(index).map(iter::IterEdges::new)
    }

    /// Returns all the nodes this node is connected to.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// let mut edges = graph.get_connids(0).expect("This does not happen");
    /// assert_eq!(edges.next(), Some(5));
    /// assert_eq!(edges.next(), None);
    /// ```
    pub fn get_connids(&'a self, index : NodeID) -> Option<iter::IterConnIds<'a, E>> {
        self.get_conn_idval(index).map(iter::IterConnIds::new)
    }

    /// Returns a list of all possible ids.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// let mut ids = graph.list_ids();
    /// assert_eq!(ids.next(), Some(0));
    /// assert_eq!(ids.next(), Some(5));
    /// assert_eq!(ids.next(), None);
    /// ```
    pub fn list_ids(&'a self) -> iter::ListIds<'a, V, E> {
        iter::ListIds::new(self.data.keys())
    }

    /// Returns a mutable version of the edge between from and to.
    ///
    /// # Examples
    /// ```
    /// use graph::Graph;
    /// let mut graph = Graph::new(
    ///             vec![(0, "A"), (5, "B")],
    ///             vec![(0, "Edge from A to B", 5)]
    ///     ).expect("This does not happen");
    ///
    /// match graph.get_edge_mut(0, 5) {
    ///     Some(t) => *t = "Hello",
    ///     None => panic!("This does not happen!"),
    /// }
    ///
    /// let mut edges = graph.get_edges(0).expect("This does not happen");
    /// assert_eq!(edges.next(), Some(&"Hello"));
    /// assert_eq!(edges.next(), None);
    /// ```
    pub fn get_edge_mut(&'a mut self, from : NodeID, to : NodeID) -> Option<&'a mut E> {
        self.data.get_mut(from).and_then(|el| el.links.get_mut(&to))
    }

    pub fn get_edge(&'a self, from : NodeID, to : NodeID) -> Option<&'a E> {
        self.data.get(from).and_then(|el| el.links.get(&to))
    }

    pub fn get_all_nodes(&'a self) -> iter::ListAllNodes<'a, V, E> {
        iter::ListAllNodes::new(self.data.values())
    }
}

use std::fmt::Debug;
impl<V : Debug, E : Debug> Graph<V, E> {
    pub fn debug(&self) {
        for id in self.list_ids() {
            println!("{:?} -> {:?}", self.get(id).unwrap(), self.get_conn_idval(id).unwrap().map(|(id, val)| (val, self.get(id).unwrap())).collect::<Vec<_>>());
        }
    }
}
