#![warn(missing_docs)]

//! Crate for gluing together logic-independent calls.
//! This crate constructs the Json data that is eventually used in the response.


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
extern crate tag_modifiers;

use newtypes::Location;
use std::error::Error;

pub use logic::ServingModel;
pub use logic::Metadata;
pub use logic::ApplicationGraph;
pub use logic::Limit;
pub use logic::RoutingError;
pub use graph::Path;

use database::Update;

mod geojson;
mod directions;
pub mod serialize;

/// Return type.
pub enum RoutingType {
    /// Return a geojson format.
    GeoJson,
    /// Return a directions format.
    Directions,
}

use RoutingType::*;

impl RoutingType {
    /// map a string to a routing type.
    pub fn from(s : &str) -> RoutingType {
        match s {
            "geojson" => GeoJson,
            _ => Directions
        }
    }
}

/// Create a string holding the Json representation of a route.
pub fn route<MF : Fn() -> Metadata>(serving_model : &ServingModel, from : &Location, to : &Location, metadata_supplier : MF, routing_type : &RoutingType, limit : &Limit)
    -> Result<String, Box<Error>> {
    info!("Creating a route from ({}, {}) to ({}, {}) with metadata {:?}", from.lon, from.lat, to.lon, to.lat, metadata_supplier());
    let mut route = Err(RoutingError::Empty);
    let mut string = String::new();
    for _ in 0..20 {
        let mut metadata = metadata_supplier();
        let rod = logic::create_rod(serving_model, from, &mut metadata).map_err(|e| format!("Rod failed: {:?}", e))?;
        string = serde_json::to_string_pretty(&geojson::into_geojson(&rod.as_path(), &serving_model.graph, &metadata.tag_converter))?;
        route = logic::close_rod(serving_model, to, &mut metadata, &rod);
        if route.is_ok() {break;}
    }
    let route = route.map_err(|e| format!("Closure failed: {:?}", e))?.0;
    use std::fs;
    use std::io::Write;
    let _ = fs::File::create("debug.json").ok().map(|mut f| f.write_all(string.as_bytes()));
    limit.improve(&route);
    let metadata = metadata_supplier();
    let converter = &metadata.tag_converter;
    Ok(match *routing_type {
        Directions => serde_json::to_string_pretty(&directions::into_directions(&route, &serving_model.graph, converter))?,
        GeoJson => serde_json::to_string_pretty(&geojson::into_geojson(&route, &serving_model.graph, converter))?,
    })
}

/// Give the given route a 27 out of 10.
pub fn rate(graph : &ApplicationGraph, route : &Path, rating : f64) -> Update {
    let edges = route.get_elements(graph).1;
    let edges_ids : Vec<_> = edges.into_iter().map(|edge| edge.edge.eid).collect();
    Update::new(edges_ids, rating)
}
