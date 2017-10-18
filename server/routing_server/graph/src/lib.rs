#![warn(missing_docs)]

//! Crate containing the graph data structure and useful algorithms, like Dijkstra's algorithm.

extern crate num;
extern crate libc;

extern crate rand;
extern crate vec_map;
extern crate util;


mod graph;
pub mod iter;
pub mod dijkstra;
mod heapdata;
mod ordering;
mod path;
pub mod error;
pub mod testgraph;

pub use graph::Graph;
pub use heapdata::HeapData;
pub use graph::{NodeID, EdgeID};
pub use path::{Path, AnnotatedPath};


pub use self::ordering::*;
