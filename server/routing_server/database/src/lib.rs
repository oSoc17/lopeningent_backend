#![warn(missing_docs)]

//! This crate provides an interface to postgresql.
//!
//! In theory, it can be replaced by Diesel. In practice, Diesel proved to be too complex to implement in one go.

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

/// Trait for converting from a sql type
pub trait Convert {
    /// Source of the conversion
    type From;
    /// Convert
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

/// Trait for debugging a query.
pub trait DebugQuery {
    /// Prints the query. Useful for debugging database_derive.
    fn debug() -> String;
}

/// Trait for queryable types.
pub trait Query : Sized {
    /// Loads all data from a table given a connection and a schema.
    /// This allows a nice balance between hard-coded tables and dynamic schemas.
    /// However, DO NOT ALLOW USER ACCESS TO THE SCHEMA! IT IS ABSOLUTELY UNSAFE!
    fn load(conn : &::postgres::Connection, schema : &str) -> Result<Vec<Self>, Box<Error>>;
}

/// A crossroad on the map.
#[derive(Query, Debug)]
#[table_name = "nodes"]
pub struct Node {
    /// Id.
    pub nid : NodeID,
    /// Longitude.
    pub lon : f64,
    /// Latitude
    pub lat : f64,
    /// List of interesting spots close by.
    pub poi_id : Vec<usize>,
}

impl Located for Node {
    fn located(&self) -> Location {
        Location::new(self.lon, self.lat)
    }
}

/// A road or footpath on the map.
#[derive(Query, Debug)]
#[table_name = "edges"]
pub struct Edge {
    /// Id.
    pub eid : EdgeID,
    /// How good the road is.
    pub rating : f32,
    /// What can be seen on the road.
    pub tags : Tags,
    /// From which crossroad the road starts.
    pub from_node : NodeID,
    /// At which crossroad the road ends.
    pub to_node : NodeID,
}

/// Point of Interest: something you should definitely visit on the map.
#[derive(Query, Debug, Serialize)]
#[table_name = "pois"]
pub struct Poi {
    /// Id.
    pub pid : usize,
    /// The name of the POI.
    pub name : String,
    /// A short description of the POI.
    pub description : Option<String>,
    /// Longitude.
    pub lon : f64,
    /// Latitude.
    pub lat : f64,
    /// A tag, useful for notifying the algorithm that this is a tourism point etc.
    pub tag : Option<String>,
}

/// I want to change the rating on the map.
pub struct Update {
    edges : Vec<EdgeID>,
    rating : f64,
}

impl Update {
    /// Creates a new update structure.
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

    /// Apply this update to the database.
    pub fn apply(&self, schema : &str, connection : &Connection, influence : f64) -> Result<(), Box<Error>> {
        let query = format!("UPDATE {}{}edges SET rating = rating * (1.0 - {3:}) + {4:} * {3:} WHERE eid IN {}", schema, if schema != "" {"."} else {""}, &Update::print(&self.edges), influence, self.rating) ;
        println!("{}", query);
        connection.execute(&query, &[])?;
        Ok(())
    }

    /// Create a connection and store the data in the database.
    pub fn store(&self, database_url : &str, schema : &str, influence : f64) -> Result<(), Box<Error>> {
        let connection = Connection::connect(database_url, TlsMode::None)?;
        self.apply(schema, &connection, influence)?;
        Ok(())
    }
}

impl Located for Poi {
    fn located(&self) -> Location {
        Location::new(self.lon, self.lat)
    }
}

/// Collection of nodes, edges, and pois.
pub struct Scheme {
    /// Nodes.
    pub nodes : Vec<Node>,
    /// Edges.
    pub edges : Vec<Edge>,
    /// Pois.
    pub pois : Vec<Poi>,
}

#[test]
fn test_slice_print() {
    let vec = vec![0, 1, 3];
    assert_eq!(Update::print(&vec), "(0, 1, 3)".to_string());
}

/// Loads a scheme from the database.
pub fn load(database_url : &str, schema : &str) -> Result<Scheme, Box<Error>> {
    let connection = Connection::connect(database_url, TlsMode::None)?;

    Ok(Scheme {
        nodes : Node::load(&connection, schema)?,
        edges : Edge::load(&connection, schema)?,
        pois : Poi::load(&connection, schema)?,
    })
}
