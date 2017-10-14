#![warn(missing_docs)]


extern crate graph;
extern crate logic;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate newtypes;
extern crate base64;
extern crate database;
#[macro_use]
extern crate log;

use newtypes::Location;
use std::error::Error;

pub use logic::Conversion;
pub use logic::Metadata;
pub use logic::ApplicationGraph;
pub use logic::Limit;
pub use graph::Path;

use database::Update;

mod geojson;
mod directions;
pub mod serialize;

pub enum RoutingType {
    GeoJson,
    Directions,
}

use RoutingType::*;

impl RoutingType {
    pub fn from(s : &str) -> RoutingType {
        match s {
            "geojson" => GeoJson,
            _ => Directions
        }
    }
}

pub fn route<MF : Fn() -> Metadata>(conversion : &Conversion, from : &Location, to : &Location, metadata_supplier : MF, routing_type : RoutingType, limit : &Limit) -> Result<String, Box<Error>> {
    info!("Creating a route from ({}, {}) to ({}, {}) with metadata {:?}", from.lon, from.lat, to.lon, to.lat, metadata_supplier());
    let mut route = None;
    let mut string = String::new();
    for _ in 0..20 {
        let mut metadata = metadata_supplier();
        let rod = logic::create_rod(conversion, from, &mut metadata).ok_or("Rod failed")?;
        string = serde_json::to_string_pretty(&geojson::into_geojson(rod.as_path(), &conversion.graph))?;
        route = logic::close_rod(conversion, to, &mut metadata, rod);
        if route.is_some() {break;}
    }
    let route = route.ok_or("Closure failed")?.0;
    use std::fs;
    use std::io::Write;
    let _ = fs::File::create("debug.json").ok().map(|mut f| f.write_all(string.as_bytes()));
    limit.improve(&route);
    Ok(match routing_type {
        Directions => serde_json::to_string_pretty(&directions::into_directions(route, &conversion.graph))?,
        GeoJson => serde_json::to_string_pretty(&geojson::into_geojson(route, &conversion.graph))?,
    })
}

pub fn rate(graph : &ApplicationGraph, route : &Path, rating : f64) -> Update {
    let edges = route.get_elements(graph).1;
    let edges_ids : Vec<_> = edges.into_iter().map(|edge| edge.edge.eid).collect();
    let update = Update::new(edges_ids, rating);
    return update;
}
