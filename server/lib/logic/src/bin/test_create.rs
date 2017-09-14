extern crate logic;
extern crate database;
extern crate newtypes;

use logic::get_graph;
use database::load;
use newtypes::{Location, Km};
use logic::Metadata;
use std::time;


fn main() {
    let graph = get_graph(load("postgresql://postgres:0987654321@localhost").unwrap()).unwrap();
    //graph.debug();
    let conversion = logic::Conversion::get_default_conversion(&graph);
    let location = Location::new(3.7, 51.0);
    let edge = conversion.get_edge(&location).unwrap();
    println!("{:?}, {:?}", graph.get(edge.edge.from_node), graph.get(edge.edge.to_node));
    let metadata =  Metadata {requested_length : Km::from_f64(20.0)};
    let now = time::Instant::now();
    let rod = logic::create_rod(&conversion, &location , &metadata).unwrap();
    println!("{:?}", rod);
    let path = logic::close_rod(&conversion, &location, &metadata, rod).unwrap().0;
    let vertices = path.get_elements(&graph).0;
    println!("{:?}", vertices);
    let duration = time::Instant::now() - now;
    println!("{}.{:09}", duration.as_secs(), duration.subsec_nanos());
}
