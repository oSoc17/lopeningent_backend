use error::Error;

use graph::graph;
use graph::graphtrait::GraphTrait;
use graph::graph::Graph;
use graph::iter;

/// Poisoned graph.
///
/// The poisoned graph is a graph where some of the edges have been replace with
/// other edges.
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
/// // Poisoning is meant to work on edges, not nodes. Therefore, all nodes in the
/// // overriding graph are actually just references to the original graph.
/// let c = &"C";
///
/// use graph::Poisoned;
/// let poisoned = Poisoned::new(
///         &vec![(0, c)],
///         &vec![],
///         &graph
///     ).expect("This does not happen");
///
/// // Now one vertex has been replaced:
/// assert_eq!(poisoned.get(0), Some(&"C"));
/// assert_eq!(poisoned.get(5), Some(&"B"));
///
/// // However...
/// // Any edge originating from the origin has been overwritten.
/// assert_eq!(poisoned.get_conn_idval(0).unwrap().count(), 0);
/// ```
pub struct Poisoned<'a, V : 'a, E : 'a + Clone, G : GraphTrait<V=V, E=E> + 'a> {
    origin : &'a G,
    overlay : Graph<&'a V, E>,
}

impl<'a, V : 'a, E : 'a + Clone, G : GraphTrait<V=V, E=E> + 'a> Poisoned<'a, V, E, G> {
    /// Create a new poisoned graph. See [Graph](../graph/index.html) for more detail.
    pub fn new(vertices : &[(usize, &'a V)], edges : &[(usize, E, usize)], origin : &'a G) -> Result<Poisoned<'a, V, E, G>, Error>
    {
        let vertexiter = vertices.iter().map(|&(n, v)| (n, v));
        let edgeiter = edges.iter().map(|&(id, ref e, to)| (id, e.clone(), to));
        let graph = graph::Graph::new(vertexiter, edgeiter);

        Ok(Poisoned {
            origin : origin,
            overlay : try!(graph),
        })
    }
}

impl<'a, V : 'a, E : 'a + Clone, G : GraphTrait<V=V, E=E> + 'a> GraphTrait for Poisoned<'a, V, E, G> {
    type V = V;
    type E = E;

    fn get(&self, index : usize) -> Option<&V> {
        self.overlay.get(index).cloned()
            .or(self.origin.get(index))
    }
    fn get_conn_idval<'b>(&'b self, index : usize) -> Option<iter::ConnIdVal<'b, E>> {
        self.overlay.get_conn_idval(index)
            .or(self.origin.get_conn_idval(index))
    }
}
