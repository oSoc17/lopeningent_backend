use graph::Path;
use graph::Graph;
use logic::{PoiNode, AnnotatedEdge};
use newtypes::{Location, Located};
use serialize;

#[derive(Serialize)]
pub struct DirectionalNode {
    pub lon : f64,
    pub lat : f64,
    pub dir : &'static str,
}

impl  DirectionalNode {
    pub fn new<L : Located>(l : &L, dir : &'static str) -> DirectionalNode {
        let q = l.located();
        DirectionalNode {
            lon : q.lon,
            lat : q.lat,
            dir : dir
        }
    }
}

fn dir_none() -> &'static str {"none"}
fn dir_forward() -> &'static str {"forward"}
fn dir_hasleft() -> &'static str {"hasleft"}
fn dir_hasright() -> &'static str {"hasright"}
fn dir_turn() -> &'static str {"turnaround"}
fn dir_left() -> &'static str {"left"}
fn dir_right() -> &'static str {"right"}

#[derive(Serialize)]
pub struct Directions {
    pub coordinates : Vec<DirectionalNode>,
    pub tag : String,
}

pub fn into_directions(path : Path, graph : &Graph<PoiNode, AnnotatedEdge>) -> Directions {
    let nodes = path.get_elements(graph).0;
    let threshold = 0.7;
    let mut res = vec![DirectionalNode::new(&nodes[0].node, dir_none())];
    for ((a, b), c) in nodes.iter().zip(nodes[1..].iter()).zip(nodes[2..].iter()) {
        if a.node.nid == c.node.nid {
            res.push(DirectionalNode::new(&b.node, dir_turn()));
            continue;
        }
        let value = angle(a, b, c);
        if value < -threshold {
            res.push(DirectionalNode::new(&b.node, dir_left()));
        } else if value > threshold {
            res.push(DirectionalNode::new(&b.node, dir_right()));
        } else {
            res.push(DirectionalNode::new(&b.node, dir_forward()));
        }
    }
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
