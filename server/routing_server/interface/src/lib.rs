extern crate graph;
extern crate logic;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate newtypes;
extern crate base64;
extern crate database;

use newtypes::Location;
use std::error::Error;

pub use logic::Conversion;
pub use logic::Metadata;

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

pub fn route(conversion : &Conversion, from : &Location, to : &Location, metadata : &Metadata, routing_type : RoutingType) -> Result<String, Box<Error>> {
    let mut route = None;
    for _ in 0..10 {
        let mut rod = logic::create_rod(conversion, from, metadata).ok_or("Rod failed")?;
        route = logic::close_rod(conversion, to, metadata, rod);
        if route.is_some() {break;}
    }
    let route = route.ok_or("Closure failed")?.0;
    Ok(match routing_type {
        Directions => serde_json::to_string_pretty(&directions::into_directions(route, &conversion.graph))?,
        GeoJson => serde_json::to_string_pretty(&geojson::into_geojson(route, &conversion.graph))?,
    })
}
