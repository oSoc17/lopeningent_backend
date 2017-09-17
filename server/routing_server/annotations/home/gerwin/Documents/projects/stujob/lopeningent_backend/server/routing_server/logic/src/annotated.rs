               :use std::sync::Arc;
               :use database::Node;
               :use database::Edge;
               :use database::Poi;
               :use newtypes::{Located, Location, Km};
               :
               :#[derive(Debug)]
               :pub struct PoiNode {
               :    pub node : Node,
               :    pub poi : Option<Arc<Vec<Poi>>>
               :}
               :
               :impl Located for PoiNode {
               :    fn located(&self) -> Location {
     3 7.9e-05 :        self.node.located()
               :    }
               :}
               :
               :#[derive(Debug)]
               :pub struct AnnotatedEdge {
               :    pub edge : Edge,
               :    pub dist : Km,
               :    pub average : Location,
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/logic/src/annotated.rs"
 * 
 *      3 7.9e-05
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
