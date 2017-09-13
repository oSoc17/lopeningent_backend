extern crate graph;
extern crate database;

use graph::Graph;
use database::{Node, Edge, Poi};
use database::load;

fn get_graph(database_url : &str) -> Result<Graph<Node, Edge>, Box<std::error::Error>> {
    let scheme = load(database_url)?;
    
}
