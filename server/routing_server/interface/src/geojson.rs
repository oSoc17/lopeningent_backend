use logic::ApplicationGraph;
use graph::Path;
use std::collections::HashSet as Set;
use database::Poi;

use tag_modifiers::Tags;
use tag_modifiers::TagModifier;

/**
    Some serialisation boilerplate.
**/
#[derive(Serialize, Deserialize)]
struct Distance {
    length : f64,
    perceived : f64
}

#[derive(Serialize)]
pub struct GeoJson<'a> {
    #[serde(rename = "type")]
    pub type_ : String,
    pub features : Vec<Feature>,
    pub pois : Vec<&'a Poi>
}

#[derive(Serialize, Deserialize)]
pub struct Feature {
    #[serde(rename = "type")]
    pub type_ : String,
    pub geometry : Geometry,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Geometry {
    LineString{coordinates : Vec<(f64, f64)>},
    Point{coordinates : (f64, f64)}
}

pub fn into_geojson<'a, T : TagModifier>(path : Path, graph : &'a ApplicationGraph, tags : &T) -> GeoJson<'a> {
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
