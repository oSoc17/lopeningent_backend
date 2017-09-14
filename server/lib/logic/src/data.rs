use graph::Graph;
use database::{Scheme, Node, Edge, Poi};
use database::load;
use newtypes::{Located, Location};
use std::sync::Arc;
use std::collections::HashMap as Map;
use buckets::{Grid, Interval};
use transform::Projector;
use newtypes::Km;
use annotated::PoiNode;
use annotated::AnnotatedEdge;
use vec_map::VecMap;

use consts::*;
use std::error::Error;
use util;
use transform;
use na;

pub fn get_graph(scheme : Scheme) -> Result<Graph<PoiNode, AnnotatedEdge>, Box<Error>> {
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
    let edges_collected : Vec<_> = {
        let indexed_nodes : VecMap<_> = poinodes.iter().map(|n| (n.node.nid, &n.node)).collect();
        edges.into_iter().map(|edge| {
        let (from, to) = (edge.from_node, edge.to_node);
        let from_loc = indexed_nodes[from];
        let to_loc = indexed_nodes[to];
        let dist = util::distance::distance_lon_lat(&from_loc.located(), &to_loc.located(), Km::from_f64(EARTH_RADIUS));
            (from, AnnotatedEdge{edge : edge, dist : dist, average : Location::average(&from_loc.located(), &to_loc.located())}, to)
        }).collect()
    };
    Ok(Graph::new(poinodes.into_iter().map(|node| (node.node.nid, node)), edges_collected)?)
}

pub struct Conversion<'a> {
    pub graph : &'a Graph<PoiNode, AnnotatedEdge>,
    pub projector : Projector,
    pub grid : Grid<&'a AnnotatedEdge>,
}

pub fn get_projector(graph : &Graph<PoiNode, AnnotatedEdge>) -> Projector {
    let avg = transform::average(graph.get_all_nodes()
        .map(|node| node.located())
        .map(|location| location.into_3d()).collect::<Vec<_>>().iter());

    Projector::new(avg, na::Vector3::new(0.0, 0.0, 1.0), Km::from_f64(EARTH_RADIUS))
}

impl<'a> Conversion<'a> {
    pub fn get_conversion(graph : &'a Graph<PoiNode, AnnotatedEdge>, projector : Projector) -> Conversion<'a> {
        let edges : Vec<_> =  graph.list_ids().flat_map(|id| graph.get_edges(id).unwrap()).collect();
        let z = Km::from_f64(0.0);
        let interval = graph.get_all_nodes()
            .map(|node| node.located())
            .map(|location| projector.map(&location.into_3d()).into())
            .map(|tuple| Interval::from(tuple,tuple, Km::from_f64(TOLERANCE)))
            .fold(Interval::from((z, z), (z, z), z), |a, b| &a + &b);
        let mut grid : Grid<&AnnotatedEdge> = Grid::from(interval, Km::from_f64(BIN_SIZE));
        for edge in edges {
            let (from, to) = (graph.get(edge.edge.from_node).unwrap(), graph.get(edge.edge.to_node).unwrap());
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

    pub fn get_default_conversion(graph : &'a Graph<PoiNode, AnnotatedEdge>) -> Conversion<'a> {
        Self::get_conversion(graph, get_projector(graph))
    }

    pub fn get_edge(&self, location : &Location) -> Option<&'a AnnotatedEdge> {
        let pos = self.projector.map(&location.into_3d()).into();
        let choices = self.grid.get(pos);
        choices.iter()
        .fold(None, |sum, edge| {
            let from = self.projector.map(&self.graph.get(edge.edge.from_node).unwrap().node.located().into_3d()).into();
            let to = self.projector.map(&self.graph.get(edge.edge.to_node).unwrap().node.located().into_3d()).into();
            let dist = util::distance::distance_to_edge(pos, from, to);
            let _ : &Option<(Km, &AnnotatedEdge)> = &sum;
            match sum {
                Some(tuple) if tuple.0 < dist => Some(tuple),
                _ => Some((dist, edge))
            }
        }).map(|(_, edge)| edge)
    }
}
