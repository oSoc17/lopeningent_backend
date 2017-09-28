use graph::{Graph, NodeID};
use database::{Scheme, Poi};

use newtypes::{Located, Location};
use std::sync::Arc;
use std::collections::HashMap as Map;
use buckets::{Grid, Interval};
use transform::Projector;
use newtypes::Km;
use newtypes::ToF64;
use annotated::PoiNode;
use annotated::AnnotatedEdge;
use annotated::ApplicationGraph;
use vec_map::VecMap;

use consts::*;
use std::error::Error;
use util;
use transform;
use na;

pub fn get_graph(scheme : Scheme) -> Result<ApplicationGraph, Box<Error>> {
    let nodes = scheme.nodes;
    let edges = scheme.edges;
    let pois = scheme.pois;
    let mut pid_arc_poi_map : Map<usize, Arc<Poi>> = Map::new();
    for poi in pois {
        pid_arc_poi_map.insert(poi.pid, Arc::new(poi));
    }
    let poinodes : Vec<_> = nodes.into_iter().map(|node| PoiNode {
        poi : node.poi_id.iter().map(|&id| pid_arc_poi_map.get(&id).map(Arc::clone)).collect(),
        node : node
        }).collect();
    let edges_collected : Vec<_> = {
        let indexed_nodes : VecMap<_> = poinodes.iter().map(|n| (n.node.nid as usize, &n.node)).collect();
        edges.into_iter().map(|edge| {
            let (from, to) = (edge.from_node, edge.to_node);
            let from_loc = indexed_nodes[from as usize];
            let to_loc = indexed_nodes[to as usize];
            let dist = util::distance::distance_lon_lat(&from_loc.located(), &to_loc.located(), Km::from_f64(EARTH_RADIUS));
                (from, AnnotatedEdge{
                    edge : edge,
                    dist : dist,
                    average : Location::average(&from_loc.located(), &to_loc.located()).into_3d(),}
                , to)
        }).collect()
    };
    Ok(Graph::new(poinodes.into_iter().map(|node| (node.node.nid, node)), edges_collected)?)
}

pub struct Conversion {
    pub graph : ApplicationGraph,
    pub projector : Projector,
    pub grid : Grid<(NodeID, NodeID)>,
}

pub fn get_projector(graph : &ApplicationGraph) -> Projector {
    let avg = transform::average(graph.get_all_nodes()
        .map(|node| node.located())
        .map(|location| location.into_3d()).collect::<Vec<_>>().iter());

    Projector::new(avg, na::Vector3::new(0.0, 0.0, 1.0), Km::from_f64(EARTH_RADIUS))
}

impl Conversion {
    pub fn get_conversion(graph : ApplicationGraph, projector : Projector) -> Conversion {
        let z = Km::from_f64(0.0);
        let interval = graph.get_all_nodes()
        .map(|node| node.located())
        .map(|location| projector.map(&location.into_3d()).into())
        .map(|tuple| Interval::from(tuple,tuple, Km::from_f64(TOLERANCE)))
        .fold(Interval::from((z, z), (z, z), z), |a, b| &a + &b);

        let mut grid : Grid<(NodeID, NodeID)> = Grid::from(interval, Km::from_f64(BIN_SIZE));

        {
            let edges : Vec<_> = graph.list_ids().flat_map(|id| graph.get_edges(id).unwrap()).collect();
            for edge in edges {
                let (from, to) = (graph.get(edge.edge.from_node).unwrap(), graph.get(edge.edge.to_node).unwrap());
                let interval = Interval::from(
                    projector.map(&from.located().into_3d()).into(),
                    projector.map(&to.located().into_3d()).into(),
                    Km::from_f64(TOLERANCE)
                );
                grid.add(interval, &(edge.edge.from_node, edge.edge.to_node));
            }
        }
        Conversion {
            graph : graph,
            projector : projector,
            grid : grid,
        }
    }

    pub fn get_default_conversion(graph : ApplicationGraph) -> Conversion {
        let projector = get_projector(&graph);
        Self::get_conversion(graph, projector)
    }

    pub fn get_edge(&self, location : &Location) -> Option<&AnnotatedEdge> {
        let pos = self.projector.map(&location.into_3d()).into();
        let choices = self.grid.get(pos);
        choices.iter()
        .fold(None, |sum, edge| {
            let edge = self.graph.get_edge(edge.0, edge.1).unwrap();
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

    pub fn debug(&self) -> String {
        let start_string = "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n".to_string();
        let mut res = self.graph.list_ids().flat_map(move |f| self.graph.get_connids(f).unwrap().map(move |t| (f, t)))
        .map(|(from, to)| (&self.graph.get(from).unwrap().node, &self.graph.get(to).unwrap().node))
        .map(|(from_node, to_node)| (
            self.projector.map(&from_node.located().into_3d()).into(),
            self.projector.map(&to_node.located().into_3d()).into()
        )).map(|((fx, fy), (tx, ty))|
            ((self.grid.get_max_x() - fx, self.grid.get_max_y() - fy),
            (self.grid.get_max_x() - tx, self.grid.get_max_y() - ty))
        )
        .map(|((from_x, from_y), (to_x, to_y))| format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:#000000;\"/>\n",
            from_x.to_f64() * 100.0,
            from_y.to_f64() * 100.0,
            to_x.to_f64() * 100.0,
            to_y.to_f64() * 100.0))
        .fold(start_string, |mut s, t| {s.push_str(&t); s})
        ;
        res.push_str("</svg>");
        res
    }
}
