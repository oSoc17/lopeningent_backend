//! Implementation of the Ford-Bellman algorithm using Pareto fronts for multidimensional cost functions.
//!
//! This implementation uses a chain of `SingleActions` to compute a path: every action contains information
//! about which node it has reached, and points to its parent to form a vector tree.
use std::collections::BinaryHeap;

use vec_map::VecMap;

use Graph;
use HeapData;
use Majorising;
use NodeID;
use std::error::Error;

use util::vec_limit::LimitedVec;

/// Used for determining whether a node is an endpoint, a restricted endpoint, or a transitional node.
#[derive(Debug, PartialEq, Eq)]
pub enum Ending {
    /// Nope.
    No,
    /// No and don't ask again.
    Kinda,
    /// Yes.
    Yes,
}

/// A single node being discovered during the algorithm.
#[derive(Debug)]
pub struct SingleAction<M : Majorising> {
    /// Index of the previous SingleAction in the vector.
    pub previous_index : usize,
    /// The value of the cost function at this point.
    pub major : M,
    /// The node that has been added to the path.
    pub node_handle : NodeID,
    /// Tag specifying whether the current path is not majorised.
    pub disabled : bool,
    /// Tag specifying whether the next result cannot be an end point.
    pub ignore : bool,
}

impl<M : Majorising> SingleAction<M> {
    /// Creates a new action
    pub fn new(previous_index : usize, node_handle : NodeID, major : M) -> SingleAction<M> {
        SingleAction {
            previous_index : previous_index,
            node_handle : node_handle,
            major : major,
            disabled : false,
            ignore : false,
        }
    }

    /// Ignores this action.
    pub fn ignore(self, ignore : bool) -> Self {
        let mut res = self;
        res.ignore = ignore;
        res
    }
}


/// Controls the functionality of the Ford-Bellman algorithm.
pub trait DijkstraControl {
    /// Vertex type
    type V;
    /// Edge type
    type E;
    /// Cost type
    type M : Majorising;
    /// Compute the cost function after adding the edge e to the path with cost value m.
    fn add_edge(&self, m : &Self::M, e : &Self::E) -> Self::M;

    /// Filter out paths that are too long.
    fn filter(&self, _ : &Self::M) -> bool {
        true
    }

    /// Hint about the cost size. This value gets added to the binary heap, for performance.
    fn hint(&self, m : &Self::M) -> u64;

    /// Use this node as an ending (Yes), midpoint (No) or a notifier that the next node cannot be an ending (Kinda)
    fn is_ending(&self, _ : &Self::V, _ : &Self::M) -> Ending {
        Ending::No
    }

    /// If true, use nodes without children as end points.
    fn yield_on_empty(&self) -> bool {
        false
    }

    /// If true, return as soon as a single path has been found, instead of until no children can be found.
    fn force_finish(&self) -> bool {
        false
    }

    /// If true, ignore the filter as long as no path has been found
    fn ignore_filter_until_ending(&self) -> bool {
        false
    }
}

/// Builder for Dijkstra's shortest path algorithm.
pub struct DijkstraBuilder<C : DijkstraControl> {
    start_node : NodeID,
    start_value : C::M,
}

impl<C : DijkstraControl> DijkstraBuilder<C>
{
    /// Create a builder from the start node and the base value (majorising
    /// costs don't have a related num::Zero implementation).
    pub fn new(start_node : NodeID, start_value : C::M) -> DijkstraBuilder<C> {
        DijkstraBuilder {
            start_node : start_node,
            start_value : start_value,
        }
    }

    /// Create a shortest path tree from a graph and a control structure.
    pub fn generate_dijkstra(self, graph : &Graph<C::V, C::E>, control : &C)
         -> Result<(Vec<SingleAction<C::M>>, Vec<usize>), Box<Error>>
    {
        // for ignore_filter_until_ending
        let mut possible_ending_found = false;

        // Save for every node a pareto front of costs.
        let mut progress : VecMap<Vec<usize>> = VecMap::new();
        progress.insert(0, vec![0]);

        //The heap
        let mut heap = BinaryHeap::new();

        // the chain of SingleActions
        let mut res_chain : LimitedVec<SingleAction<C::M>>= LimitedVec::new(Vec::new());

        // The actual ending points of the chain.
        let mut res_endpoints : Vec<usize> = Vec::new();
        heap.push(HeapData::new(0, self.start_node, &self.start_value, control));
        res_chain.push(SingleAction::new(0, self.start_node, self.start_value))?;
        while let Some(data) = heap.pop() {
            if res_chain.inner()[data.index].disabled {
                // This node is is majorised. Cannot be part of the shortest path.
                continue;
            }
            let ending = control.is_ending(graph.get(data.node).unwrap(), &res_chain.inner()[data.index].major);
            if ending != Ending::No {
                // We have found a possible ending
                possible_ending_found = true;
            }

            // In case we need to quickly finish.
            let ignore_next_step = control.force_finish() && {
                if ending == Ending::Yes && ! res_chain.inner()[data.index].ignore {
                    // We have found an ending.
                    break;
                }
                ending == Ending::Kinda
            };

            if let Some(iter) = graph.get_conn_idval(data.node) {
                for (next_node, next_edge) in iter {
                    // Compute the cost of adding the edge.
                    let next_major = control.add_edge(&res_chain.inner()[data.index].major, next_edge);

                    // Apply the filter
                    if (!control.ignore_filter_until_ending() || possible_ending_found) && ! control.filter(&next_major) {
                        continue;
                    }

                    // Disable any end points that it majorises.
                    let h_vec = progress.entry(next_node as usize).or_insert_with(Vec::new);
                    for &e in h_vec.iter() {
                        if res_chain.inner()[e].major.majorises_strict(&next_major) {
                            res_chain.get_mut(e).unwrap().disabled = true;
                        }
                    }

                    // Remove any majorised nodes
                    h_vec.retain(|&e| ! res_chain.inner()[e].disabled);

                    // If no majorising actions can be found, add this to the
                    if  h_vec.iter().find(|&&e| next_major.majorises(&res_chain.inner()[e].major)) == None {
                        let index = res_chain.inner().len();
                        heap.push(HeapData::new(index, next_node, &next_major, control));
                        h_vec.push(index);
                        res_chain.push(SingleAction::new(data.index, next_node, next_major)
                            .ignore(ignore_next_step))?
                    }
                }
            }
        }

        let mut res_chain = res_chain.into_inner();

        // Filter endpoints based on whether they're ending and the point before is not a possible ending.
        // This is to prevent turnaround points in routes.
        for &index in progress.iter().flat_map(|(_, v)| v.iter()) {
            let action = &res_chain[index];
            if control.is_ending(graph.get(action.node_handle).unwrap(), &action.major) == Ending::Yes {
                let prev_action = &res_chain[action.previous_index];
                if control.is_ending(graph.get(prev_action.node_handle).unwrap(), &prev_action.major) == Ending::No {
                    res_endpoints.push(index);
                }
            }
        }
        // Select all points that have no successor.
        if control.yield_on_empty() {
            // Disable all points with a successor
            for index in 0..res_chain.len() {
                if ! res_chain[index].disabled {
                    let previous_index = res_chain[index].previous_index;
                    res_chain[previous_index].disabled = true;
                }
            }
            // Retrieve the rest.
            for (index, element) in res_chain.iter().enumerate() {
                if ! element.disabled {
                    res_endpoints.push(index)
                }
            }
        }
        Ok((res_chain, res_endpoints))
    }
}

use Path;
use AnnotatedPath;


/// Trace back a chain from an ending point, retrieving a path.
pub fn into_nodes<M : Majorising>(res_chain : &Vec<SingleAction<M>>, start : usize) -> Path {
    into_annotated_nodes_functor(res_chain, start, |_| ()).as_path()
}

/// Trace back a chain from an ending point, retrieving a path with annotations.
pub fn into_annotated_nodes<M : Majorising + Clone>(res_chain : &Vec<SingleAction<M>>, start : usize) -> AnnotatedPath<M> {
    into_annotated_nodes_functor(res_chain, start, |m| m.clone())
}

fn into_annotated_nodes_functor<M : Majorising, T, F : Fn(&M) -> T>(res_chain : &Vec<SingleAction<M>>, start : usize, f : F) -> AnnotatedPath<T> {
    let mut res = Vec::new();
    let mut index = start;
    loop {
        // simple tracing.
        res.push((res_chain[index].node_handle, f(&res_chain[index].major)));
        if index != res_chain[index].previous_index {
            index = res_chain[index].previous_index;
        }
        else {
            break;
        }
    }
    res.reverse();
    AnnotatedPath::new(res)
}
