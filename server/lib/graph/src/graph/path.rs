use graph::Graph;
use graph::NodeID;

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
}

pub struct AnnotatedPath<D>(Vec<(NodeID, D)>);

impl<D> AnnotatedPath<D> {
    pub fn new(vec : Vec<(NodeID, D)>) -> AnnotatedPath<D> {
        AnnotatedPath(vec)
    }

    pub fn get_elements<'a, V : 'a, E : 'a>(&self, graph : &'a Graph<V, E>) -> (Vec<&'a V>, Vec<&'a E>) {
        (
            self.0.iter().map(|i| graph.get(i.0).unwrap()).collect(),
            self.0.iter().zip(self.0.iter().skip(1)).map(|(i, j)| graph.get_edge(i.0, j.0).unwrap()).collect()
        )
    }
}
