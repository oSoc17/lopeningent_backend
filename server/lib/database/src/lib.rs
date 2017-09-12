extern crate postgres;
#[macro_use]
extern crate database_derive;
use postgres::TlsMode;
use postgres::Connection;
use std::error::Error;


pub trait DebugQuery {
    fn debug() -> String;
}

pub trait Query : Sized {
    fn load(conn : &::postgres::Connection) -> Result<Vec<Self>, Box<Error>>;
}

#[derive(Query, Debug)]
#[table_name = "lopeningent.nodes"]
pub struct Node {
    pub nid : i32,
    pub lon : f64,
    pub lat : f64,
}

#[derive(Query, Debug)]
#[table_name = "lopeningent.edges"]
pub struct Edge {
    pub eid : i32,
    pub rating : f32,
    pub tags : Vec<String>,
    pub from_node : i32,
    pub to_node : i32,
}

#[derive(Query, Debug)]
#[table_name = "lopeningent.pois"]
pub struct Poi {
    pub pid : i32,
    pub name : String,
    pub description : Option<String>,
    pub lon : f64,
    pub lat : f64,
    pub tag : Option<String>,
}

pub struct Scheme {
    pub nodes : Vec<Node>,
    pub edges : Vec<Edge>,
    pub pois : Vec<Poi>,
}

pub fn load(database_url : &str) -> Result<Scheme, Box<Error>> {
    let connection = Connection::connect(database_url, TlsMode::None)?;

    println!("{}", Node::debug());
    println!("{}", Edge::debug());
    println!("{}", Poi::debug());

    Ok(Scheme {
        nodes : Node::load(&connection)?,
        edges : Edge::load(&connection)?,
        pois : Poi::load(&connection)?,
    })
}
