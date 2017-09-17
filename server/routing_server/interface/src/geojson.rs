use logic::AnnotatedEdge;
use logic::PoiNode;
use graph::Path;
use graph::Graph;

/**
    Some serialisation boilerplate.
**/
#[derive(Serialize, Deserialize)]
struct Distance {
    length : f64,
    perceived : f64
}

#[derive(Serialize, Deserialize)]
pub struct GeoJson {
    #[serde(rename = "type")]
    pub type_ : String,
    pub features : Vec<Feature>,

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

pub fn into_geojson(path : Path, graph : &Graph<PoiNode, AnnotatedEdge>) -> GeoJson {
    let (nodes, edges) = path.get_elements(graph);
    GeoJson {
        type_ : "FeatureCollection".to_string(),
        features : vec![Feature {
            type_ : "Feature".to_string(),
            geometry : Geometry::LineString {
                coordinates : nodes.into_iter().map(|node| (node.node.lon, node.node.lat)).collect()
            }
        }]
    }
}
