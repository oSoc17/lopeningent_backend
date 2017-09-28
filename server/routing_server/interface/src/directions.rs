use graph::Path;

use database::Poi;
use logic::{ApplicationGraph, PoiNode};
use newtypes::{Located};
use serialize;

#[derive(Serialize)]
pub struct DirectionalNode<'a> {
    pub lon : f64,
    pub lat : f64,
    pub dir : &'static str,
    pub pois : Option<Vec<&'a Poi>>,
}

impl<'a>  DirectionalNode<'a> {
    pub fn new(poinode : &'a PoiNode, dir : &'static str) -> DirectionalNode<'a> {
        let q = poinode.node.located();
        use std::ops::Deref;
        DirectionalNode {
            lon : q.lon,
            lat : q.lat,
            dir : dir,
            pois : poinode.poi.as_ref().map(|vec| vec.iter().map(|arc| arc.deref()).collect())
        }
    }
}

fn dir_none() -> &'static str {"none"}
fn dir_forward() -> &'static str {"forward"}
fn dir_turn() -> &'static str {"turnaround"}
fn dir_left() -> &'static str {"left"}
fn dir_right() -> &'static str {"right"}

#[derive(Serialize)]
pub struct Directions<'a> {
    pub coordinates : Vec<DirectionalNode<'a>>,
    pub tag : String,
}

pub fn into_directions<'a>(path : Path, graph : &'a ApplicationGraph) -> Directions<'a> {
    let nodes = path.get_elements(graph).0;
    let threshold = 0.7;
    let mut res = vec![DirectionalNode::new(&nodes[0], dir_none())];
    for ((a, b), c) in nodes.iter().zip(nodes[1..].iter()).zip(nodes[2..].iter()) {
        if a.node.nid == c.node.nid {
            res.push(DirectionalNode::new(&b, dir_turn()));
            continue;
        }
        let value = angle(a, b, c);
        if value < -threshold {
            res.push(DirectionalNode::new(&b, dir_left()));
        } else if value > threshold {
            res.push(DirectionalNode::new(&b, dir_right()));
        } else {
            res.push(DirectionalNode::new(&b, dir_forward()));
        }
    }
    res.push(DirectionalNode::new(&nodes[nodes.len() - 1], dir_none()));
    Directions {
        coordinates : res,
        tag : serialize::to_string(&path),
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
