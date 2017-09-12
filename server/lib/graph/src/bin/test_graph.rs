extern crate graph;

use graph::testgraph::create_testgraph;
fn main() {
    let graph = create_testgraph(3, 3, |x, y| (x, y), |n, m| 1.0).unwrap();
    graph.debug();
}
