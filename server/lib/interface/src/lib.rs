extern crate graph;
extern crate logic;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate newtypes;
extern crate base64;

use newtypes::Location;
pub use logic::Conversion;
pub use logic::Metadata;

mod geojson;
mod directions;
mod serialize;

pub enum RoutingType {
    GeoJson,
    Directions,
}

pub fn route(conversion : &Conversion, from : &Location, to : &Location, metadata : &Metadata, routing_type : RoutingType) -> Result<String, ()> {
    let rod = logic::create_rod(conversion, from, metadata).ok_or(())?;
    let route = logic::close_rod(conversion, to, metadata, rod).ok_or(())?.0;
    Ok(match routing_type {
        Directions => serde_json::to_string_pretty(&directions::into_directions(route, &conversion.graph)).map_err(|_| ())?,
        GeoJson => serde_json::to_string_pretty(&geojson::into_geojson(route, &conversion.graph)).map_err(|_| ())?,
    })
}
