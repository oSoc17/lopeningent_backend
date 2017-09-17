               ://! Structure used for putting data on a heap, for Dijkstra purposes.
               ://! It also inverts the comparison operator, which is useful since the
               ://! Binary Heap data structure in Rust yields all data in high-to-low order.
               :
               :use std::cmp::Ordering;
               :use graph::ordering::Majorising;
               :use graph::NodeID;
               :use graph::dijkstra::DijkstraControl;
               :use num::traits::WrappingSub;
               :
               :/// HeapData struct
               :
               :#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
               :pub struct HeapData
               :{
 50543  1.3352 :    pub hint : u64,
   218  0.0058 :    pub index : usize,
               :    pub node: NodeID,
               :}
               :
               :impl HeapData
               :{
               :    /// Create a new HeapData struct
               :    pub fn new<C : DijkstraControl>(index : usize, node: NodeID, major : &C::M, control : &C) -> HeapData {
   553  0.0146 :        HeapData {
               :            index : index,
               :            node: node,
               :            hint : 0.wrapping_sub(&control.hint(major)),
               :        }
               :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/graph/src/graph/heapdata.rs"
 * 
 *  51314  1.3555
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
