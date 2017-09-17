               :
               :use std::collections::BinaryHeap;
               :use std::rc::Rc;
               :use std::cell::RefCell;
               :use std::collections::HashMap;
               :use std::collections::HashSet;
               :use std::ops::Add;
               :
               :use vec_map::VecMap;
               :
               :use graph::Graph;
               :use graph::HeapData;
               :use graph::Majorising;
               :use graph::NodeID;
               :
               :#[derive(Debug)]
               :pub struct SingleAction<M : Majorising> {
               :    pub previous_index : usize,
               :    pub major : M,
               :    pub node_handle : NodeID,
               :    pub disabled : bool,
               :}
               :
               :impl<M : Majorising> SingleAction<M> {
               :    pub fn new(previous_index : usize, node_handle : NodeID, major : M) -> SingleAction<M> {
               :        SingleAction {
               :            previous_index : previous_index,
               :            node_handle : node_handle,
               :            major : major,
               :            disabled : false,
               :        }
               :    }
               :}
               :
               :
               :
               :pub trait DijkstraControl {
               :    type V;
               :    type E;
               :    type M : Majorising;
               :    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M;
               :    fn filter(&self, m : &Self::M) -> bool {
               :        true
               :    }
               :    fn hint(&self, m : &Self::M) -> u64;
               :    fn is_ending(&self, v : &Self::V, m : &Self::M) -> bool {
               :        false
               :    }
               :    fn yield_on_empty(&self) -> bool {
               :        false
               :    }
               :    fn force_finish(&self) -> bool {
               :        false
               :    }
               :}
               :
               :pub struct DijkstraBuilder<C : DijkstraControl> {
               :    start_node : NodeID,
               :    start_value : C::M,
               :}
               :
               :use std::fmt::Debug;
               :impl<C : DijkstraControl> DijkstraBuilder<C>
               :where C::M : Debug, C::V : Debug, C::E : Debug
               :{
               :    pub fn new(start_node : NodeID, start_value : C::M) -> DijkstraBuilder<C> {
               :        DijkstraBuilder {
               :            start_node : start_node,
               :            start_value : start_value,
               :        }
               :    }
               :
               :    pub fn generate_dijkstra(self, graph : &Graph<C::V, C::E>, control : &C)
               :         -> (Vec<SingleAction<C::M>>, Vec<usize>)
               :    {
               :        let mut progress : VecMap<Vec<usize>> = VecMap::new();
               :        progress.insert(0, vec![0]);
               :        let mut heap = BinaryHeap::new();
               :        let mut res_chain : Vec<SingleAction<C::M>>= Vec::new();
               :        let mut res_endpoints : Vec<usize> = Vec::new();
               :        heap.push(HeapData::new(0, self.start_node, &self.start_value, control));
               :        res_chain.push(SingleAction::new(0, self.start_node, self.start_value));
  1541  0.0407 :        while let Some(data) = heap.pop() {
 14358  0.3793 :            if res_chain[data.index].disabled {
               :                continue;
               :            }
     5 1.3e-04 :            if control.force_finish()
               :                && control.is_ending(graph.get(data.node).unwrap(), &res_chain[data.index].major) {
               :                break;
               :            }
               :            if let Some(iter) = graph.get_conn_idval(data.node) {
               :                for (next_node, next_edge) in iter {
               :                    let next_major = control.add_edge(&res_chain[data.index].major, &next_edge);
  2164  0.0572 :                    if ! control.filter(&next_major) {
               :                        continue;
               :                    }
   221  0.0058 :                    let mut h_vec = progress.entry(next_node as usize).or_insert_with(Vec::new);
               :                    //println!("Inserting {:?} into {:?}... (from {} to {}) ", next_major, h_vec.iter().map(|&c| &res_chain[c].major).collect::<Vec<_>>(), data.node , next_node);
 22112  0.5841 :                    for &e in h_vec.iter() {
 20082  0.5305 :                        if res_chain[e].major.majorises_strict(&next_major) {
  8632  0.2280 :                            res_chain[e].disabled = true;
               :                        }
               :                    }
337733  8.9216 :                    h_vec.retain(|&e| ! res_chain[e].disabled);
   806  0.0213 :                    if h_vec.iter().filter(|&&e| next_major.majorises(&res_chain[e].major)).next() == None {
               :                        //print!("Replacement. ");
               :                        let index = res_chain.len();
   381  0.0101 :                        heap.push(HeapData::new(index, next_node, &next_major, control));
               :                        h_vec.push(index);
               :                        res_chain.push(SingleAction::new(data.index, next_node, next_major) )
               :                    }
               :                    //println!("Got {:?}", h_vec.iter().map(|&c| &res_chain[c].major).collect::<Vec<_>>());
               :                }
               :            }
               :        }
   188  0.0050 :        for &index in progress.iter().flat_map(|(n, v)| v.iter()) {
               :            let action = &res_chain[index];
   245  0.0065 :            if control.is_ending(graph.get(action.node_handle).unwrap(), &action.major) {
               :                let prev_action = &res_chain[action.previous_index];
               :                if !control.is_ending(graph.get(prev_action.node_handle).unwrap(), &prev_action.major)  {
               :                    res_endpoints.push(index);
               :                }
               :            }
               :        }
               :
               :        if control.yield_on_empty() {
               :            for index in 0..res_chain.len() {
   337  0.0089 :                if ! res_chain[index].disabled {
    21 5.5e-04 :                    let previous_index = res_chain[index].previous_index;
    23 6.1e-04 :                    res_chain[previous_index].disabled = true;
               :                }
               :            }
               :            for index in 0..res_chain.len() {
   268  0.0071 :                if ! res_chain[index].disabled {
               :                    res_endpoints.push(index)
               :                }
               :            }
               :        }
               :        (res_chain, res_endpoints)
               :    }
               :}
               :
               :use graph::Path;
               :use graph::AnnotatedPath;
               :
               :pub fn into_nodes<M : Majorising>(res_chain : &Vec<SingleAction<M>>, start : usize) -> Path {
               :    let mut res = Vec::new();
               :    let mut index = start;
               :    loop {
               :
               :        res.push(res_chain[index].node_handle);
               :        if index != res_chain[index].previous_index {
               :            index = res_chain[index].previous_index;
               :        }
               :        else {
               :            break;
               :        }
               :    }
               :    res.reverse();
               :    return Path::new(res);
               :}
               :
               :// TODO remove code duplication.
               :pub fn into_annotated_nodes<M : Majorising + Clone>(res_chain : &Vec<SingleAction<M>>, start : usize) -> AnnotatedPath<M> { /* graph::graph::dijkstra::into_annotated_nodes::h15ce79a5a529efdc total:     36 9.5e-04 */
               :    let mut res = Vec::new();
               :    let mut index = start;
               :    loop {
               :
    32 8.5e-04 :        res.push((res_chain[index].node_handle, res_chain[index].major.clone()));
               :        if index != res_chain[index].previous_index {
               :            index = res_chain[index].previous_index;
               :        }
               :        else {
               :            break;
               :        }
               :    }
               :    res.reverse();
               :    return AnnotatedPath::new(res);
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/graph/src/graph/dijkstra.rs"
 * 
 * 409149 10.8082
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
