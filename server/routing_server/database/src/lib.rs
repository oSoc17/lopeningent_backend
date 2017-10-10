#![warn(missing_docs)]


extern crate postgres;
#[macro_use]
extern crate database_derive;
extern crate newtypes;
extern crate graph;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tag_modifiers;

use postgres::TlsMode;
use postgres::Connection;
use std::error::Error;
use newtypes::{Located, Location};
use graph::{NodeID, EdgeID};
pub use tag_modifiers::*;


pub trait Convert {
    type From;
    fn convert(from : Self::From) -> Self;
}

macro_rules! default_impl {
    ($($type : ty, $from : ty);*) => {
        $(
            impl Convert for $type {
                type From = $from;
                fn convert(from : Self::From) -> Self {
                    from as $type
                }
            }
        )*
    };
}

default_impl!(i32, i32; i64, i64; u64, i32; usize, i32; String, String; f32, f32; f64, f64);

impl<T : Convert> Convert for Option<T> {
    type From = Option<T>;
    fn convert(from : Self::From) -> Self {
        from
    }
}

impl<T : Convert> Convert for Vec<T> {
    type From = Vec<T::From>;
    fn convert(from : Self::From) -> Self {
        from.into_iter().map(|t| T::convert(t)).collect()
    }
}

impl Convert for Tags {
    type From = Vec<String>;
    fn convert(t : Vec<String>) -> Tags {
        Tags::from(t)
    }
}


pub trait DebugQuery {
    fn debug() -> String;
}

pub trait Query : Sized {
    fn load(conn : &::postgres::Connection, schema : &str) -> Result<Vec<Self>, Box<Error>>;
}

#[derive(Query, Debug)]
#[table_name = "nodes"]
pub struct Node {
    pub nid : NodeID,
    pub lon : f64,
    pub lat : f64,
    pub poi_id : Vec<usize>,
}

impl Located for Node {
    fn located(&self) -> Location {
        Location::new(self.lon, self.lat)
    }
}

#[derive(Query, Debug)]
#[table_name = "edges"]
pub struct Edge {
    pub eid : EdgeID,
    pub rating : f32,
    pub tags : Tags,
    pub from_node : NodeID,
    pub to_node : NodeID,
}

#[derive(Query, Debug, Serialize)]
#[table_name = "pois"]
pub struct Poi {
    pub pid : usize,
    pub name : String,
    pub description : Option<String>,
    pub lon : f64,
    pub lat : f64,
    pub tag : Option<String>,
}

pub struct Update {
    edges : Vec<EdgeID>,
    rating : f64,
}

impl Update {
    pub fn new(edges : Vec<EdgeID>, rating : f64) -> Update {
        Update {
            edges : edges,
            rating : rating,
        }
    }

    fn print(slice : &[EdgeID]) -> String {
        if slice.len() == 0 {
            return "()".to_string();
        }
        let mut s = String::new();
        s.push_str("(");
        s.push_str(&slice[0].to_string());
        for id in &slice[1..] {
            s.push_str(", ");
            s.push_str(&id.to_string());
        }
        s.push_str(")");
        return s;
    }

    pub fn apply(&self, connection : &Connection, influence : f64) -> Result<(), Box<Error>> {
        let option_schema = option_env!("SCHEMA");
        let query = format!("UPDATE {}{}edges SET rating = rating * (1.0 - {3:}) + {4:} * {3:} WHERE eid IN {}", option_schema.as_ref().map_or("", |x| &**x), if option_schema.is_some() {"."} else {""}, &Update::print(&self.edges), influence, self.rating) ;
        println!("{}", query);
        connection.execute(&query, &[])?;
        Ok(())
    }

    pub fn store(&self, database_url : &str, influence : f64) -> Result<(), Box<Error>> {
        let connection = Connection::connect(database_url, TlsMode::None)?;
        self.apply(&connection, influence)?;
        Ok(())
    }
}

impl Located for Poi {
    fn located(&self) -> Location {
        Location::new(self.lon, self.lat)
    }
}

pub struct Scheme {
    pub nodes : Vec<Node>,
    pub edges : Vec<Edge>,
    pub pois : Vec<Poi>,
}

#[test]
fn test_slice_print() {
    let vec = vec![0, 1, 3];
    assert_eq!(Update::print(&vec), "(0, 1, 3)".to_string());
}

pub fn load(database_url : &str, schema : &str) -> Result<Scheme, Box<Error>> {
    let connection = Connection::connect(database_url, TlsMode::None)?;
    use std::io;
    use std::io::Write;

    Ok(Scheme {
        nodes : Node::load(&connection, schema)?,
        edges : Edge::load(&connection, schema)?,
        pois : Poi::load(&connection, schema)?,
    })
}
