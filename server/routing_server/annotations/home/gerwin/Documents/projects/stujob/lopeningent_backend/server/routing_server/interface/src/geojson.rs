               :use logic::AnnotatedEdge;
               :use logic::PoiNode;
               :use graph::Path;
               :use graph::Graph;
               :
               :/**
               :    Some serialisation boilerplate.
               :**/
               :#[derive(Serialize, Deserialize)]
               :struct Distance {
               :    length : f64,
               :    perceived : f64
               :}
               :
               :#[derive(Serialize, Deserialize)]
               :pub struct GeoJson {
               :    #[serde(rename = "type")]
               :    pub type_ : String,
               :    pub features : Vec<Feature>,
               :
               :}
               :
               :#[derive(Serialize, Deserialize)]
               :pub struct Feature {
               :    #[serde(rename = "type")]
               :    pub type_ : String,
               :    pub geometry : Geometry,
               :}
               :
               :#[derive(Serialize, Deserialize)]
               :#[serde(tag = "type")]
               :pub enum Geometry {
               :    LineString{coordinates : Vec<(f64, f64)>},
               :    Point{coordinates : (f64, f64)}
               :}
               :
     1 2.6e-05 :pub fn into_geojson(path : Path, graph : &Graph<PoiNode, AnnotatedEdge>) -> GeoJson {
               :    let (nodes, edges) = path.get_elements(graph);
               :    GeoJson {
               :        type_ : "FeatureCollection".to_string(),
               :        features : vec![Feature {
               :            type_ : "Feature".to_string(),
               :            geometry : Geometry::LineString {
               :                coordinates : nodes.into_iter().map(|node| (node.node.lon, node.node.lat)).collect()
               :            }
               :        }]
               :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/interface/src/geojson.rs"
 * 
 *      1 2.6e-05
 */


/* 
 * Command line: opannotate --source --output-dir=annotations ./target/release/routing_server 
 * 
 * Interpretation of command line:
 * Output annotated source file with samples
 * Output all files
 * 
 * CPU: Intel Ivy Bridge microarchitecture, speed 3100 MHz (estimated)
 * Counted CPU_CLK_UNHALTED events (Clock cycles when not halted) with a unit mask of 0x00 (No unit mask) count 90000
 */
