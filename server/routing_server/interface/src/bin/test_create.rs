extern crate logic;
extern crate interface;
extern crate database;
extern crate serde_json;
extern crate newtypes;
extern crate tag_modifiers;

use logic::get_graph;
use database::{load, TagConverter};
use newtypes::{Location, Km};
use logic::Metadata;
use std::time;
use std::io;
use std::io::Write;

use std::sync::Arc;


fn main() {
    let graph = get_graph(load("postgresql://postgres:0987654321@localhost", "lopeningent2").unwrap()).unwrap();
    let serving_model = logic::ServingModel::get_default_serving_model(graph);
    let location = Location::new(3.7, 51.0);
    let metadata =  Metadata {requested_length : Km::from_f64(20.0), tag_converter : TagConverter::default(), original_route : None};
    let now = time::Instant::now();
    let serving_model = Arc::new(serving_model);
    let res = interface::route(&*serving_model, &location, &location , || metadata.clone(), &interface::RoutingType::Directions, &logic::Limit::new(Arc::clone(&serving_model),  1.0)).unwrap();
    let duration = time::Instant::now() - now;
    println!("{}", res);
    let _ = writeln!(io::stderr(), "{}.{:09}", duration.as_secs(), duration.subsec_nanos());
}
