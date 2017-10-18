/// Some utility functionality for routing.
use graph::Path;

use database::{Tags, TagConverter};
use database::TagModifier;
use annotated::ApplicationGraph;

use newtypes::Km;

use std::f64;

use consts::*;

/// Information about the route.
#[derive(Default, Debug, Clone)]
pub struct Metadata {
    /// Preferred length of the route.
    pub requested_length : Km,
    /// Information about which tags are liked.
    pub tag_converter : TagConverter,
    /// The original route in case of return routing.
    pub original_route : Option<Path>,
}

impl Metadata {
    /// Add a tag to the metadata.
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

/// Retrieve the length of a path.
pub fn path_length(path : &Path, graph : &ApplicationGraph) -> Km {
    (path.get_elements(graph).1)
        .into_iter().map(|x| x.dist).fold(Km::from_f64(0.0), |x, y| x + y)
}
