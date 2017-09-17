               :use graph::{Graph, NodeID};
               :use database::{Scheme, Node, Edge, Poi};
               :use database::load;
               :use newtypes::{Located, Location};
               :use std::sync::Arc;
               :use std::collections::HashMap as Map;
               :use buckets::{Grid, Interval};
               :use transform::Projector;
               :use newtypes::Km;
               :use newtypes::ToF64;
               :use annotated::PoiNode;
               :use annotated::AnnotatedEdge;
               :use vec_map::VecMap;
               :
               :use consts::*;
               :use std::error::Error;
               :use util;
               :use transform;
               :use na;
               :
               :pub fn get_graph(scheme : Scheme) -> Result<Graph<PoiNode, AnnotatedEdge>, Box<Error>> { /* logic::data::get_graph::he78e41799e804a03 total:   1370  0.0362 */
               :    let nodes = scheme.nodes;
               :    let edges = scheme.edges;
               :    let pois = scheme.pois;
               :    let mut loc_poi_map : Map<Location, Vec<Poi>> = Map::new();
               :    for poi in pois {
               :        loc_poi_map.entry(poi.located()).or_insert_with(Vec::new).push(poi);
               :    }
               :    let mut loc_arc_poi_map = Map::new();
               :    for (key, value) in loc_poi_map {
               :        loc_arc_poi_map.insert(key, Arc::new(value));
               :    }
     4 1.1e-04 :    let poinodes : Vec<_> = nodes.into_iter().map(|node| PoiNode {
    12 3.2e-04 :        poi : loc_arc_poi_map.get(&node.located()).map(|n| n.clone()),
     5 1.3e-04 :        node : node
               :        }).collect();
               :    let edges_collected : Vec<_> = {
               :        let indexed_nodes : VecMap<_> = poinodes.iter().map(|n| (n.node.nid as usize, &n.node)).collect();
               :        edges.into_iter().map(|edge| {
               :        let (from, to) = (edge.from_node, edge.to_node);
               :        let from_loc = indexed_nodes[from as usize];
               :        let to_loc = indexed_nodes[to as usize];
     8 2.1e-04 :        let dist = util::distance::distance_lon_lat(&from_loc.located(), &to_loc.located(), Km::from_f64(EARTH_RADIUS));
     2 5.3e-05 :            (from, AnnotatedEdge{edge : edge, dist : dist, average : Location::average(&from_loc.located(), &to_loc.located())}, to)
               :        }).collect()
               :    };
               :    Ok(Graph::new(poinodes.into_iter().map(|node| (node.node.nid, node)), edges_collected)?)
               :}
               :
               :pub struct Conversion {
               :    pub graph : Graph<PoiNode, AnnotatedEdge>,
               :    pub projector : Projector,
               :    pub grid : Grid<(NodeID, NodeID)>,
               :}
               :
               :pub fn get_projector(graph : &Graph<PoiNode, AnnotatedEdge>) -> Projector {
               :    let avg = transform::average(graph.get_all_nodes()
               :        .map(|node| node.located())
     9 2.4e-04 :        .map(|location| location.into_3d()).collect::<Vec<_>>().iter());
               :
               :    Projector::new(avg, na::Vector3::new(0.0, 0.0, 1.0), Km::from_f64(EARTH_RADIUS))
               :}
               :
               :impl Conversion {
               :    pub fn get_conversion(graph : Graph<PoiNode, AnnotatedEdge>, projector : Projector) -> Conversion { /* logic::data::Conversion::get_conversion::hd8692147558b0e9b total:   1027  0.0271 */
               :        let z = Km::from_f64(0.0);
               :        let interval = graph.get_all_nodes()
               :        .map(|node| node.located())
     5 1.3e-04 :        .map(|location| projector.map(&location.into_3d()).into())
     6 1.6e-04 :        .map(|tuple| Interval::from(tuple,tuple, Km::from_f64(TOLERANCE)))
    16 4.2e-04 :        .fold(Interval::from((z, z), (z, z), z), |a, b| &a + &b);
               :
               :        let mut grid : Grid<(NodeID, NodeID)> = Grid::from(interval, Km::from_f64(BIN_SIZE));
               :
               :        {
               :            let edges : Vec<_> = graph.list_ids().flat_map(|id| graph.get_edges(id).unwrap()).collect();
     3 7.9e-05 :            for edge in edges {
     1 2.6e-05 :                let (from, to) = (graph.get(edge.edge.from_node).unwrap(), graph.get(edge.edge.to_node).unwrap());
     4 1.1e-04 :                let interval = Interval::from(
     6 1.6e-04 :                    projector.map(&from.located().into_3d()).into(),
    15 4.0e-04 :                    projector.map(&to.located().into_3d()).into(),
     2 5.3e-05 :                    Km::from_f64(TOLERANCE)
               :                );
    16 4.2e-04 :                grid.add(interval, &(edge.edge.from_node, edge.edge.to_node));
               :            }
               :        }
               :        Conversion {
               :            graph : graph,
               :            projector : projector,
               :            grid : grid,
               :        }
               :    }
               :
               :    pub fn get_default_conversion(graph : Graph<PoiNode, AnnotatedEdge>) -> Conversion { /* logic::data::Conversion::get_default_conversion::h55224af5ddad0b86 total:     36 9.5e-04 */
               :        let projector = get_projector(&graph);
               :        Self::get_conversion(graph, projector)
               :    }
               :
               :    pub fn get_edge(&self, location : &Location) -> Option<&AnnotatedEdge> { /* logic::data::Conversion::get_edge::h4633ddf0b43ed321 total:   1145  0.0302 */
               :        let pos = self.projector.map(&location.into_3d()).into();
               :        let choices = self.grid.get(pos);
    56  0.0015 :        choices.iter()
               :        .fold(None, |sum, edge| {
               :            let edge = self.graph.get_edge(edge.0, edge.1).unwrap();
    41  0.0011 :            let from = self.projector.map(&self.graph.get(edge.edge.from_node).unwrap().node.located().into_3d()).into();
    43  0.0011 :            let to = self.projector.map(&self.graph.get(edge.edge.to_node).unwrap().node.located().into_3d()).into();
    54  0.0014 :            let dist = util::distance::distance_to_edge(pos, from, to);
               :            let _ : &Option<(Km, &AnnotatedEdge)> = &sum;
               :            match sum {
    19 5.0e-04 :                Some(tuple) if tuple.0 < dist => Some(tuple),
               :                _ => Some((dist, edge))
               :            }
               :        }).map(|(_, edge)| edge)
               :    }
               :
               :    pub fn debug(&self) -> String {
               :        let start_string = "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n".to_string();
               :        let mut res = self.graph.list_ids().flat_map(move |f| self.graph.get_connids(f).unwrap().map(move |t| (f, t)))
               :        .map(|(from, to)| (&self.graph.get(from).unwrap().node, &self.graph.get(to).unwrap().node))
               :        .map(|(from_node, to_node)| (
               :            self.projector.map(&from_node.located().into_3d()).into(),
               :            self.projector.map(&to_node.located().into_3d()).into()
               :        )).map(|((fx, fy), (tx, ty))|
               :            ((self.grid.get_max_x() - fx, self.grid.get_max_y() - fy),
               :            (self.grid.get_max_x() - tx, self.grid.get_max_y() - ty))
               :        )
               :        .map(|((from_x, from_y), (to_x, to_y))| format!(
               :            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:#000000;\"/>\n",
               :            from_x.to_f64() * 100.0,
               :            from_y.to_f64() * 100.0,
               :            to_x.to_f64() * 100.0,
               :            to_y.to_f64() * 100.0))
               :        .fold(start_string, |mut s, t| {s.push_str(&t); s})
               :        ;
               :        res.push_str("</svg>");
               :        res
               :    }
               :}
/* 
 * Total samples for file : "/home/gerwin/Documents/projects/stujob/lopeningent_backend/server/routing_server/logic/src/data.rs"
 * 
 *    327  0.0086
 */


/* 
 * Command line: opannotate --source --output-dir=annotations ./target/release/routing_server 
 * 
 * Interpretation of command line:
 * Output annotated source file with samples
 * Output all files
 * 
 * CPU: Intel Ivy Bridge microarchitecture, speed 3100 MHz (estimated)
 * Counted CPU_CLK_UNHALTED events (Clock cycles when not halted) with a unit mask of 0x00 (No unit mask) count 90000
 */
