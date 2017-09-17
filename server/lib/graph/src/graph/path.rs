use graph::Graph;
use graph::NodeID;

use vec_map::VecMap;

#[derive(Debug)]
pub struct Path(Vec<NodeID>);

impl Path {
    pub fn new(vec : Vec<NodeID>) -> Path {
        Path(vec)
    }

    pub fn get_elements<'a, V : 'a, E : 'a>(&self, graph : &'a Graph<V, E>) -> (Vec<&'a V>, Vec<&'a E>) {
        (
            self.0.iter().map(|&i| graph.get(i).unwrap()).collect(),
            self.0.iter().zip(self.0.iter().skip(1)).map(|(&i, &j)| graph.get_edge(i, j).unwrap()).collect()
        )
    }

    pub fn first(&self) -> NodeID{
        self.0[0]
    }

    pub fn last(&self) -> NodeID {
        self.0[self.0.len() - 1]
    }

    pub fn join(self, other : Path) -> Path {
        let last = other.last();
        Path::new((self.0).into_iter().take_while(|&n| n != last).chain((other.0).into_iter().rev()).collect())
    }

    pub fn get_indices(&self) -> &[NodeID] {
        &self.0
    }
}

#[derive(Debug)]
pub struct AnnotatedPath<D>(Vec<(NodeID, D)>);

impl<D> AnnotatedPath<D> {
    pub fn new(vec : Vec<(NodeID, D)>) -> AnnotatedPath<D> {
        AnnotatedPath(vec)
    }

    pub fn first(&self) -> &(NodeID, D) {
        &self.0[0]
    }

    pub fn last(&self) -> &(NodeID, D) {
        &self.0[self.0.len() - 1]
    }

    pub fn into_map(&self) -> VecMap<&D> {
        self.0.iter().map(|tuple| (tuple.0 as usize, &tuple.1)).collect()
    }

    pub fn into_path(&self) -> Path {
        Path::new((self.0).iter().map(|&(n, _)| n).collect())
    }

    pub fn get_elements<'a, V : 'a, E : 'a>(&self, graph : &'a Graph<V, E>) -> (Vec<&'a V>, Vec<&'a E>) {
        (
            self.0.iter().map(|i| graph.get(i.0).unwrap()).collect(),
            self.0.iter().zip(self.0.iter().skip(1)).map(|(i, j)| graph.get_edge(i.0, j.0).unwrap()).collect()
        )
    }
}
