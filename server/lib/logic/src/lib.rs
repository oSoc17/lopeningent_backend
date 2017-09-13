extern crate graph;
extern crate database;
extern crate util;
extern crate newtypes;
extern crate transform;
extern crate buckets;
extern crate nalgebra as na;

use graph::Graph;
use database::{Scheme, Node, Edge, Poi};
use database::load;
use newtypes::{Located, Location};
use std::sync::Arc;
use std::collections::HashMap as Map;
use buckets::{Grid, Interval};
use transform::Projector;
use util::distance::distance_to_edge;
use newtypes::Km;

const EARTH_RADIUS : f64 = 6731.0;
const BIN_SIZE : f64 = 1.0;
const TOLERANCE : f64 = 0.1;

#[derive(Debug)]
pub struct PoiNode {
    pub node : Node,
    pub poi : Option<Arc<Vec<Poi>>>
}

impl Located for PoiNode {
    fn located(&self) -> Location {
        self.node.located()
    }
}

pub fn get_graph(scheme : Scheme) -> Result<Graph<PoiNode, Edge>, Box<std::error::Error>> {
    let nodes = scheme.nodes;
    let edges = scheme.edges;
    let pois = scheme.pois;
    let mut loc_poi_map : Map<Location, Vec<Poi>> = Map::new();
    for poi in pois {
        loc_poi_map.entry(poi.located()).or_insert_with(Vec::new).push(poi);
    }
    let mut loc_arc_poi_map = Map::new();
    for (key, value) in loc_poi_map {
        loc_arc_poi_map.insert(key, Arc::new(value));
    }
    let poinodes : Vec<_> = nodes.into_iter().map(|node| PoiNode {
            poi : loc_arc_poi_map.get(&node.located()).map(|n| n.clone()),
            node : node
        }).collect();
    Ok(Graph::new(poinodes.into_iter().map(|node| (node.node.nid, node)), edges.into_iter().map(|edge| {
        let (from, to) = (edge.from_node, edge.to_node);
            (from, edge, to)
        }))?)
}

pub struct Conversion<'a> {
    graph : &'a Graph<PoiNode, Edge>,
    projector : Projector,
    grid : Grid<&'a Edge>,
}

pub fn get_projector(graph : &Graph<PoiNode, Edge>) -> Projector {
    let avg = transform::average(graph.get_all_nodes()
        .map(|node| node.located())
        .map(|location| location.into_3d()).collect::<Vec<_>>().iter());

    Projector::new(avg, na::Vector3::new(0.0, 0.0, 1.0), Km::from_f64(EARTH_RADIUS))
}

impl<'a> Conversion<'a> {
    pub fn get_conversion(graph : &'a Graph<PoiNode, Edge>, projector : Projector) -> Conversion<'a> {
        let edges : Vec<_> =  graph.list_ids().flat_map(|id| graph.get_edges(id).unwrap()).collect();
        let z = Km::from_f64(0.0);
        let interval = graph.get_all_nodes()
            .map(|node| node.located())
            .map(|location| projector.map(&location.into_3d()).into())
            .map(|tuple| Interval::from(tuple,tuple, Km::from_f64(TOLERANCE)))
            .fold(Interval::from((z, z), (z, z), z), |a, b| &a + &b);
        let mut grid : Grid<&Edge> = Grid::from(interval, Km::from_f64(BIN_SIZE));
        for edge in edges {
            let (from, to) = (graph.get(edge.from_node).unwrap(), graph.get(edge.to_node).unwrap());
            let interval = Interval::from(
                projector.map(&from.located().into_3d()).into(),
                projector.map(&to.located().into_3d()).into(),
                Km::from_f64(TOLERANCE)
            );
            grid.add(interval, &edge);
        }
        Conversion {
            graph : graph,
            projector : projector,
            grid : grid,
        }
    }

    pub fn get_default_conversion(graph : &'a Graph<PoiNode, Edge>) -> Conversion<'a> {
        Self::get_conversion(graph, get_projector(graph))
    }

    pub fn get_edge(&self, location : Location) -> Option<&'a Edge> {
        let pos = self.projector.map(&location.into_3d()).into();
        let choices = self.grid.get(pos);
        choices.iter()
        .fold(None, |sum, edge| {
            let from = self.projector.map(&self.graph.get(edge.from_node).unwrap().node.located().into_3d()).into();
            let to = self.projector.map(&self.graph.get(edge.to_node).unwrap().node.located().into_3d()).into();
            let dist = distance_to_edge(pos, from, to);
            let _ : &Option<(Km, &Edge)> = &sum;
            match sum {
                Some(tuple) if tuple.0 < dist => Some(tuple),
                _ => Some((dist, edge))
            }
        }).map(|(_, edge)| edge)
    }
}
