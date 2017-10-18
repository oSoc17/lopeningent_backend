extern crate graph;

use graph::dijkstra::{DijkstraControl, Ending};
use graph::testgraph::*;
use graph::dijkstra::{DijkstraBuilder, SingleAction};
use std::f64;

use graph::dijkstra::into_nodes;
fn main() {
    let graph = create_testgraph(10, 10, |x, y| (x, y), |n, m|
        //(1.0, 1.0/((n / 10 + 1) as f64).exp())
        1.0
        //if n == m + 1 || m == n + 1 {(0.0, (n/10) as f64)} else {((n%10) as f64, 0.0)}
    ).unwrap();
    graph.debug();
    println!();

    let controller = Controller;
    let builder = DijkstraBuilder::new(0, (0.0, 0.0));
    let (actions, entries) = builder.generate_dijkstra(&graph, &controller).unwrap();
    println!("{:#?}", actions);
    println!("{:#?}", entries.iter().map(|index| &actions[*index]).collect::<Vec<_>>());
    println!("{}", actions.len());
    println!("{}", entries.len());
    for entry in entries {
        println!("{:?}", into_nodes(&actions, entry).get_elements(&graph).0);
    }

}

pub struct Controller;

impl DijkstraControl for Controller {
    type V = (usize, usize);
    type E = f64;
    type M = (f64, f64);
    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M {
        (m.0 + e, m.1 + m.0 + e)
    }
    fn filter(&self, m : &Self::M) -> bool {
        m.1 <= 10.0
    }
    fn hint(&self, m : &Self::M) -> u64 {
        (m.1 * 1000.0) as u64
    }
    fn is_ending(&self, v : &Self::V, m : &Self::M) -> Ending {
        Ending::No//v == &(0, 9)
    }
    fn yield_on_empty(&self) -> bool {
        true
    }
}
