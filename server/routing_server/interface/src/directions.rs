use graph::Path;

use database::Poi;
use logic::{ApplicationGraph, PoiNode};
use newtypes::{Located};
use serialize;

use std::collections::HashSet as Set;

use tag_modifiers::Tags;
use tag_modifiers::TagModifier;

#[derive(Serialize)]
pub struct DirectionalNode {
    pub lon : f64,
    pub lat : f64,
    #[serde(rename = "c")]
    pub dir : &'static str
}

impl DirectionalNode {
    pub fn new<'a, T : TagModifier>(poinode : &'a PoiNode, dir : &'static str, tags : &T) -> (DirectionalNode, Option<Vec<&'a Poi>>) {
        let q = poinode.node.located();
        use std::ops::Deref;
        (DirectionalNode {
            lon : q.lon,
            lat : q.lat,
            dir : dir
        }, poinode.poi.as_ref()
            .map(|vec| vec.iter()
                .map(|arc| arc.deref())
                .filter(|poi| tags.tag_modifier(&Tags::from(poi.tag.as_ref())) > 0.0).collect()))
    }
}

fn dir_none() -> &'static str {"none"}
fn dir_forward() -> &'static str {"forward"}
fn dir_turn() -> &'static str {"turnaround"}
fn dir_left() -> &'static str {"left"}
fn dir_right() -> &'static str {"right"}

#[derive(Serialize)]
pub struct Directions<'a> {
    pub coordinates : Vec<DirectionalNode>,
    pub tag : String,
    pub pois : Vec<&'a Poi>
}

pub fn into_directions<'a, T : TagModifier>(path : Path, graph : &'a ApplicationGraph, tags : &T) -> Directions<'a> {
    let nodes = path.get_elements(graph).0;
    let threshold = 0.7;
    let mut res = vec![DirectionalNode::new(&nodes[0], dir_none(), tags)];
    for ((a, b), c) in nodes.iter().zip(nodes[1..].iter()).zip(nodes[2..].iter()) {
        if a.node.nid == c.node.nid {
            res.push(DirectionalNode::new(&b, dir_turn(), tags));
            continue;
        }
        let value = angle(a, b, c);
        let topush = if value < -threshold {
            DirectionalNode::new(&b, dir_left(), tags)
        } else if value > threshold {
            DirectionalNode::new(&b, dir_right(), tags)
        } else {
            DirectionalNode::new(&b, dir_forward(), tags)
        };
        if graph.get_edges(b.node.nid).unwrap().count() <= 2 {
            res.push(DirectionalNode::new(&b, dir_none(), tags));
        } else {
            res.push(topush);
        }
    }
    res.push(DirectionalNode::new(&nodes[nodes.len() - 1], dir_none(), tags));

    let mut set = Set::new();
    let mut poi_vec = Vec::new();
    for poi in res.iter().filter_map(|node| (node.1).as_ref()).flat_map(|vec| vec.iter().map(|arc| &**arc)) {
        if set.insert(poi.pid) {
            poi_vec.push(poi);
        }
    }

    Directions {
        coordinates : res.into_iter().map(|(a, b)| a).collect(),
        tag : serialize::to_string(&path),
        pois : poi_vec,
    }
}

fn angle(a : &PoiNode, b : &PoiNode, c : &PoiNode) -> f64 {
    let a_loc = a.node.located().into_3d();
    let b_loc = b.node.located().into_3d();
    let c_loc = c.node.located().into_3d();
    let ab_vec = (b_loc - a_loc).normalize();
    let bc_vec = (c_loc - b_loc).normalize();
    ab_vec.cross(&bc_vec).dot(&b_loc)
}
