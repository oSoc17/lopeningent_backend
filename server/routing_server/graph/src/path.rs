//! A data structure containing a path through the graph. This path only contains
//! node indices and thus needs the referenced graph in order to keep existing.

use Graph;
use NodeID;

use vec_map::VecMap;
use std::collections::HashSet;


/// Path data structure
#[derive(Debug, Clone)]
pub struct Path(Vec<NodeID>);

impl Path {
    /// Create a new path.
    pub fn new(vec : Vec<NodeID>) -> Path {
        Path(vec)
    }

    /// Get all elements of the graph from the
    pub fn get_elements<'a, V : 'a, E : 'a>(&self, graph : &'a Graph<V, E>) -> (Vec<&'a V>, Vec<&'a E>) {
        (
            self.0.iter().map(|&i| graph.get(i).unwrap()).collect(),
            self.0.iter().zip(self.0.iter().skip(1)).map(|(&i, &j)| graph.get_edge(i, j).unwrap()).collect()
        )
    }

    /// Get the path's starting point.
    pub fn first(&self) -> NodeID{
        self.0[0]
    }

    /// Get the path's ending point.
    pub fn last(&self) -> NodeID {
        self.0[self.0.len() - 1]
    }

    /// Join two paths together.
    ///
    /// The result of this operation is self, limited until self hits the other path's ending point, appended with the
    ///
    /// #[Example]
    /// ```
    /// let start = Path::new(vec![1, 2, 3, 4, 5]);
    /// let end = Path::new(vec![6, 2, 8, 3]);
    /// let joined = start.join(end);
    /// assert_eq!(joined.get_indices(), &vec![1, 2, 3, 8, 2, 6]);
    /// ```
    pub fn join(self, other : Path) -> Path {
        let last = other.last();
        Path::new((self.0).into_iter().take_while(|&n| n != last).chain((other.0).into_iter().rev()).collect())
    }

    /// Concatenate two paths, without checking node overlap.
    pub fn append(self, other : Path) -> Path {
        Path::new((self.0).into_iter().chain((other.0).into_iter()).collect())
    }

    /// Retrieve all indices, in order, of this path.
    pub fn get_indices(&self) -> &[NodeID] {
        &self.0
    }

    /// Limit the path until it hits node with id last_index.
    pub fn truncate(&mut self, last_index : NodeID) -> bool {
        let size = match (self.0).iter().enumerate().find(|&(_, &i)| i == last_index) {
            Some((n, _)) => n + 1,
            None => return false,
        };
        (self.0).truncate(size);
        true
    }

    /// Sort the indices based on where they first occur in the graph.
    pub fn get_first_occuring(&self, indices : &[NodeID]) -> Vec<NodeID> {
        let mut res = Vec::new();
        let mut to_hit : HashSet<_> = indices.iter().cloned().collect();
        for &node_id in &self.0 {
            if to_hit.contains(&node_id) {
                to_hit.remove(&node_id);
                res.push(node_id);
            }
        }
        res
    }
}

/// A path, annotated with a tag per node. Usually this tag holds the distance to the starting point.
#[derive(Debug)]
pub struct AnnotatedPath<D>(Vec<(NodeID, D)>);

impl<D> AnnotatedPath<D> {
    /// Create a new annotated path.
    pub fn new(vec : Vec<(NodeID, D)>) -> AnnotatedPath<D> {
        AnnotatedPath(vec)
    }

    /// Retrieve the starting node.
    pub fn first(&self) -> &(NodeID, D) {
        &self.0[0]
    }

    /// retrieve the ending node.
    pub fn last(&self) -> &(NodeID, D) {
        &self.0[self.0.len() - 1]
    }

    /// Only keep the nodes for which the distance satisfies the function.
    pub fn get_path_filtered<F : Fn(&D) -> bool>(&self, f : F) -> Path {
        Path::new((self.0).iter().filter(|t| f(&t.1)).map(|t| t.0).collect())
    }

    /// Retrieve a NodeID -> D map.
    pub fn as_map(&self) -> VecMap<&D> {
        self.0.iter().map(|tuple| (tuple.0 as usize, &tuple.1)).collect()
    }

    /// Throw away distances.
    pub fn as_path(&self) -> Path {
        Path::new((self.0).iter().map(|&(n, _)| n).collect())
    }

    /// See Path.
    pub fn get_elements<'a, V : 'a, E : 'a>(&self, graph : &'a Graph<V, E>) -> (Vec<&'a V>, Vec<&'a E>) {
        (
            self.0.iter().map(|i| graph.get(i.0).unwrap()).collect(),
            self.0.iter().zip(self.0.iter().skip(1)).map(|(i, j)| graph.get_edge(i.0, j.0).unwrap()).collect()
        )
    }
}
