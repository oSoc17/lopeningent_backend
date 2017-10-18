use logic::ApplicationGraph;
use graph::Path;
use std::collections::HashSet as Set;
use database::Poi;

use tag_modifiers::Tags;
use tag_modifiers::TagModifier;
/// Module for converting a path to a GeoJson-compatible format.

/**
    Some serialisation boilerplate.
**/
#[derive(Serialize, Deserialize)]
struct Distance {
    length : f64,
    perceived : f64
}

/// The struct.
#[derive(Serialize)]
pub struct GeoJson<'a> {
    #[serde(rename = "type")]
    type_ : String,
    features : Vec<Feature>,
    pois : Vec<&'a Poi>
}

#[derive(Serialize, Deserialize)]
struct Feature {
    #[serde(rename = "type")]
    type_ : String,
    geometry : Geometry,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Geometry {
    LineString{coordinates : Vec<(f64, f64)>},
    Point{coordinates : (f64, f64)}
}

/// Construct the return type.
pub fn into_geojson<'a, T : TagModifier>(path : &Path, graph : &'a ApplicationGraph, tags : &T) -> GeoJson<'a> {
    let (nodes, _) = path.get_elements(graph);
    let mut set = Set::new();
    let mut poi_vec = Vec::new();
    for poi in nodes.iter().filter_map(|node| node.poi.as_ref()).flat_map(|vec| vec.iter().map(|arc| &**arc)) {
        if tags.tag_modifier(&Tags::from(poi.tag.as_ref())) > 0.0 && set.insert(poi.pid) {
            poi_vec.push(poi);
        }
    }
    GeoJson {
        type_ : "FeatureCollection".to_string(),
        pois : poi_vec,
        features : vec![Feature {
            type_ : "Feature".to_string(),
            geometry : Geometry::LineString {
                coordinates : nodes.into_iter().map(|node| (node.node.lon, node.node.lat)).collect()
            }
        }]
    }
}
