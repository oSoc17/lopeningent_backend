extern crate postgres;
#[macro_use]
extern crate database_derive;
extern crate newtypes;
use postgres::TlsMode;
use postgres::Connection;
use std::error::Error;
use newtypes::{Located, Location};

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

default_impl!(i32, i32; i64, i64; usize, i32; String, String; f32, f32; f64, f64);

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
    tourism : bool,
    water : bool,
    park : bool,
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
    pub nid : usize,
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
    pub eid : usize,
    pub rating : f32,
    pub tags : Tags,
    pub from_node : usize,
    pub to_node : usize,
}

#[derive(Query, Debug)]
#[table_name = "lopeningent.pois"]
pub struct Poi {
    pub pid : usize,
    pub name : String,
    pub description : Option<String>,
    pub lon : f64,
    pub lat : f64,
    pub tag : Option<String>,
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
