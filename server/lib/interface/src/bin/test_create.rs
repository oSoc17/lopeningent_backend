extern crate logic;
extern crate interface;
extern crate database;
extern crate serde_json;
extern crate newtypes;

use logic::get_graph;
use database::load;
use newtypes::{Location, Km};
use logic::Metadata;
use std::time;
use std::io;
use std::io::Write;


fn main() {
    let graph = get_graph(load("postgresql://postgres:0987654321@localhost").unwrap()).unwrap();
    //graph.debug();
    let conversion = logic::Conversion::get_default_conversion(&graph);
    use std::fs;
    /*
    let mut file = fs::File::create("/home/gerwin/debug.svg").unwrap();
    file.write(&conversion.debug().into_bytes()).unwrap();
    */
    let location = Location::new(3.7126612, 51.0475082);
    let edge = conversion.get_edge(&location).unwrap();
    // println!("{:?}, {:?}", graph.get(edge.edge.from_node), graph.get(edge.edge.to_node));
    let metadata =  Metadata {requested_length : Km::from_f64(20.0), water : 1.0, park : 1.0, tourism : -1.0};
    let now = time::Instant::now();
    let res = interface::route(&conversion, &location, &location , &metadata, interface::RoutingType::Directions).unwrap();
    // println!("{:?}", rod);
    // println!("{:?}", vertices);
    let duration = time::Instant::now() - now;
    println!("{}", res);
    writeln!(io::stderr(), "{}.{:09}", duration.as_secs(), duration.subsec_nanos());
    //println!();
    //println!();

}
