
use std::collections::BinaryHeap;






use vec_map::VecMap;

use graph::Graph;
use graph::HeapData;
use graph::Majorising;
use graph::NodeID;

#[derive(Debug, PartialEq, Eq)]
pub enum Ending {
    No,
    Kinda,
    Yes,
}

#[derive(Debug)]
pub struct SingleAction<M : Majorising> {
    pub previous_index : usize,
    pub major : M,
    pub node_handle : NodeID,
    pub disabled : bool,
    pub ignore : bool,
}

impl<M : Majorising> SingleAction<M> {
    pub fn new(previous_index : usize, node_handle : NodeID, major : M) -> SingleAction<M> {
        SingleAction {
            previous_index : previous_index,
            node_handle : node_handle,
            major : major,
            disabled : false,
            ignore : false,
        }
    }

    pub fn ignore(self, ignore : bool) -> Self {
        let mut res = self;
        res.ignore = ignore;
        res
    }
}



pub trait DijkstraControl {
    type V;
    type E;
    type M : Majorising;
    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M;
    fn filter(&self, _ : &Self::M) -> bool {
        true
    }
    fn hint(&self, m : &Self::M) -> u64;
    fn is_ending(&self, _ : &Self::V, _ : &Self::M) -> Ending {
        Ending::No
    }
    fn yield_on_empty(&self) -> bool {
        false
    }
    fn force_finish(&self) -> bool {
        false
    }
}

pub struct DijkstraBuilder<C : DijkstraControl> {
    start_node : NodeID,
    start_value : C::M,
}

use std::fmt::Debug;
impl<C : DijkstraControl> DijkstraBuilder<C>
where C::M : Debug, C::V : Debug, C::E : Debug
{
    pub fn new(start_node : NodeID, start_value : C::M) -> DijkstraBuilder<C> {
        DijkstraBuilder {
            start_node : start_node,
            start_value : start_value,
        }
    }

    pub fn generate_dijkstra(self, graph : &Graph<C::V, C::E>, control : &C)
         -> (Vec<SingleAction<C::M>>, Vec<usize>)
    {
        let mut progress : VecMap<Vec<usize>> = VecMap::new();
        progress.insert(0, vec![0]);
        let mut heap = BinaryHeap::new();
        let mut res_chain : Vec<SingleAction<C::M>>= Vec::new();
        let mut res_endpoints : Vec<usize> = Vec::new();
        heap.push(HeapData::new(0, self.start_node, &self.start_value, control));
        res_chain.push(SingleAction::new(0, self.start_node, self.start_value));
        while let Some(data) = heap.pop() {
            if res_chain[data.index].disabled {
                continue;
            }
            let ignore_next_step = control.force_finish() && {
                let ending = control.is_ending(graph.get(data.node).unwrap(), &res_chain[data.index].major);
                if ending == Ending::Yes && ! res_chain[data.index].ignore {
                    break;
                }
                ending == Ending::Kinda
            };


            if let Some(iter) = graph.get_conn_idval(data.node) {
                for (next_node, next_edge) in iter {
                    let next_major = control.add_edge(&res_chain[data.index].major, &next_edge);
                    if ! control.filter(&next_major) {
                        continue;
                    }
                    let mut h_vec = progress.entry(next_node as usize).or_insert_with(Vec::new);
                    //println!("Inserting {:?} into {:?}... (from {} to {}) ", next_major, h_vec.iter().map(|&c| &res_chain[c].major).collect::<Vec<_>>(), data.node , next_node);
                    for &e in h_vec.iter() {
                        if res_chain[e].major.majorises_strict(&next_major) {
                            res_chain[e].disabled = true;
                        }
                    }
                    h_vec.retain(|&e| ! res_chain[e].disabled);
                    if h_vec.iter().filter(|&&e| next_major.majorises(&res_chain[e].major)).next() == None {
                        //print!("Replacement. ");
                        let index = res_chain.len();
                        heap.push(HeapData::new(index, next_node, &next_major, control));
                        h_vec.push(index);
                        res_chain.push(SingleAction::new(data.index, next_node, next_major)
                            .ignore(ignore_next_step))
                    }
                    //println!("Got {:?}", h_vec.iter().map(|&c| &res_chain[c].major).collect::<Vec<_>>());
                }
            }
        }
        for &index in progress.iter().flat_map(|(_, v)| v.iter()) {
            let action = &res_chain[index];
            if control.is_ending(graph.get(action.node_handle).unwrap(), &action.major) == Ending::Yes {
                let prev_action = &res_chain[action.previous_index];
                if control.is_ending(graph.get(prev_action.node_handle).unwrap(), &prev_action.major) == Ending::No {
                    res_endpoints.push(index);
                }
            }
        }

        if control.yield_on_empty() {
            for index in 0..res_chain.len() {
                if ! res_chain[index].disabled {
                    let previous_index = res_chain[index].previous_index;
                    res_chain[previous_index].disabled = true;
                }
            }
            for index in 0..res_chain.len() {
                if ! res_chain[index].disabled {
                    res_endpoints.push(index)
                }
            }
        }
        (res_chain, res_endpoints)
    }
}

use graph::Path;
use graph::AnnotatedPath;

pub fn into_nodes<M : Majorising>(res_chain : &Vec<SingleAction<M>>, start : usize) -> Path {
    let mut res = Vec::new();
    let mut index = start;
    loop {

        res.push(res_chain[index].node_handle);
        if index != res_chain[index].previous_index {
            index = res_chain[index].previous_index;
        }
        else {
            break;
        }
    }
    res.reverse();
    return Path::new(res);
}

// TODO remove code duplication.
pub fn into_annotated_nodes<M : Majorising + Clone>(res_chain : &Vec<SingleAction<M>>, start : usize) -> AnnotatedPath<M> {
    let mut res = Vec::new();
    let mut index = start;
    loop {

        res.push((res_chain[index].node_handle, res_chain[index].major.clone()));
        if index != res_chain[index].previous_index {
            index = res_chain[index].previous_index;
        }
        else {
            break;
        }
    }
    res.reverse();
    return AnnotatedPath::new(res);
}
