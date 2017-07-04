#![warn(missing_docs)]

//! Crate containing the graph data structure and useful algorithms, like Dijkstra's algorithm.

extern crate num;
extern crate libc;
extern crate newtypes;
extern crate rand;
extern crate util;
extern crate vec_map;

mod graph;
#[cfg(test)]
mod tests;
pub mod error;

pub use graph::*;
