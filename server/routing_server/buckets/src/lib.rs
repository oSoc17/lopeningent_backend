#![warn(missing_docs)]

//! Crate that implements a form of spacial buckets.
//!
//! This crate hosts a grid and an interval struct. You can add intervals to the grid, with data, and then query
//! all intervals in the grid that contain a specific point.
//!
//! The grid might return more intervals than just the ones that have hit the point. Every interval overlapping with the square containing the point will be returned. 

extern crate newtypes;
extern crate num;

mod grid;
mod interval;

pub use grid::*;
pub use interval::*;
