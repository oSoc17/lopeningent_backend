#![warn(missing_docs)]
//! Logic crate. As the name suggests, contains all logic to glue together the basic building blocks in other crates.
//! This crate is the firewall between graphs and interfaces.

extern crate graph;
extern crate database;
extern crate util;
extern crate newtypes;
extern crate transform;
extern crate buckets;
extern crate vec_map;
extern crate nalgebra as na;
#[macro_use]
extern crate log;

mod data;
mod annotated;
mod routing;
mod consts;
mod limit;

pub use data::get_graph;
pub use data::ServingModel;
pub use annotated::{AnnotatedEdge, PoiNode, ApplicationGraph};
pub use consts::*;
pub use routing::{Distance, Metadata};
pub use routing::{create_rod, close_rod};
pub use routing::RoutingError;
pub use limit::Limit;
