
use graph::{Path, AnnotatedPath};
use data::Conversion;

use graph::dijkstra::DijkstraBuilder;
use graph::dijkstra::{DijkstraControl, Ending};
use graph::dijkstra::into_annotated_nodes;
use graph::Majorising;
use graph::NodeID;

use database::{Tags, TagConverter};
use database::TagModifier;
use annotated::{PoiNode, AnnotatedEdge, ApplicationGraph};


use newtypes::{Location, Located};
use newtypes::ToF64;
use newtypes::Km;

use std::f64;
use std::io;
use std::io::Write;
use std::collections::HashSet;

use std::sync::atomic::Ordering;

use vec_map::VecMap;

use na;

use util;
use util::selectors::Selector;

use consts::*;


#[derive(Default, Debug, Clone)]
pub struct Metadata {
    pub requested_length : Km,
    pub tag_converter : TagConverter,
    pub original_route : Option<Path>,
}

impl Metadata {
    pub fn add(&mut self, tag : &str, size : f64) {
        self.tag_converter.add(tag, size)
    }
}

impl TagModifier for Metadata {
    fn tag_modifier(&self, tag : &Tags) -> f64 {
        self.tag_converter.tag_modifier(tag)
    }
    fn tag_bounds() -> (f64, f64) {
        (ABS_MINIMUM, ABS_MAXIMUM)
    }
}

pub fn path_length(path : &Path, graph : &ApplicationGraph) -> Km {
    (path.get_elements(graph).1)
        .into_iter().map(|x| x.dist).fold(Km::from_f64(0.0), |x, y| x + y)
}
