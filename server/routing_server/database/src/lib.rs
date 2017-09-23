extern crate postgres;
#[macro_use]
extern crate database_derive;
extern crate newtypes;
extern crate graph;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use postgres::TlsMode;
use postgres::Connection;
use std::error::Error;
use newtypes::{Located, Location};
use graph::{NodeID, EdgeID};

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

impl Convert for Tags {
    type From = Vec<String>;
    fn convert(t : Vec<String>) -> Tags {
        let mut res = Tags::default();
        for i in t {
            match i.as_ref() {
                "tourism" => res.tourism = true,
                "water" => res.water = true,
                "park" => res.park = true,
                _ => ()
            }
        }
        res
    }
}

#[derive(Debug, Default)]
pub struct Tags {
    // Okay, this sucks, I admit.
    pub tourism : bool,
    pub water : bool,
    pub park : bool,
}

impl Tags {
    pub fn trues(&self) -> usize {
        (if self.tourism {1} else {0})
        + (if self.water {1} else {0})
        + (if self.park {1} else {0})
    }
}

/*impl Convert<Vec<String>> for Tags {
    fn convert(t : Vec<String>) -> Tags {
        let mut res = Tags::default();
        for i in t {
            match i.as_ref() {
                "tourism" => res.tourism = true,
                "water" => res.water = true,
                "park" => res.park = true,
                _ => ()
            }
        }
        res
    }
}*/

pub trait DebugQuery {
    fn debug() -> String;
}

pub trait Query : Sized {
    fn load(conn : &::postgres::Connection) -> Result<Vec<Self>, Box<Error>>;
}

#[derive(Query, Debug)]
#[table_name = "lopeningent.nodes"]
pub struct Node {
    pub nid : NodeID,
    pub lon : f64,
    pub lat : f64,
}

impl Located for Node {
    fn located(&self) -> Location {
        Location::new(self.lon, self.lat)
    }
}

#[derive(Query, Debug)]
#[table_name = "lopeningent.edges"]
pub struct Edge {
    pub eid : EdgeID,
    pub rating : f32,
    pub tags : Tags,
    pub from_node : NodeID,
    pub to_node : NodeID,
}

#[derive(Query, Debug, Serialize)]
#[table_name = "lopeningent.pois"]
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
        let query = format!("UPDATE lopeningent.edges SET rating = rating * (1.0 - {1:}) + {2:} * {1:} WHERE eid IN {}", &Update::print(&self.edges), influence, self.rating) ;
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

pub fn load(database_url : &str) -> Result<Scheme, Box<Error>> {
    let connection = Connection::connect(database_url, TlsMode::None)?;
    use std::io;
    use std::io::Write;
    writeln!(io::stderr(), "{}", Node::debug());
    writeln!(io::stderr(), "{}", Edge::debug());
    writeln!(io::stderr(), "{}", Poi::debug());

    Ok(Scheme {
        nodes : Node::load(&connection)?,
        edges : Edge::load(&connection)?,
        pois : Poi::load(&connection)?,
    })
}
