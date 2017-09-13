extern crate logic;
extern crate database;
extern crate newtypes;

use logic::get_graph;
use database::load;


fn main() {
    let graph = get_graph(load("postgresql://postgres:0987654321@localhost").unwrap()).unwrap();
    //graph.debug();
    let conversion = logic::Conversion::get_default_conversion(&graph);
    let edge = conversion.get_edge(newtypes::Location::new(3.7, 51.0)).unwrap();
    println!("{:?}, {:?}", graph.get(edge.from_node), graph.get(edge.to_node));

}
