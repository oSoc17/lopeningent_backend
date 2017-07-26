from server.logic.graph.graph import Graph
from server.logic.graph.util import distance
from server.model.node import Node
from collections import namedtuple
from server.logic.graph.util import distance
import server.database as db
import logging

def load():
    logging.info("requesting nodes and edges from the database")
    nodelist, edgelist = db.get_graph_data()
    logging.debug("nodes: %s", nodelist)
    logging.debug("edges: %s", edgelist)
    return Graph(nodelist, edgelist)

def get_edges():
    _, edgelist = db.get_graph_data()
    edges = dict()

    for edge in edgelist:
        edges[edge.id] = edge

    return edges

def project(graph, projector):
    """ 
    Creates an enhanced graph with xy coordinates from a graph 
    """
    graph.map_graph(Mapper(projector), lambda x: x)


class Mapper(namedtuple("Mapper", "projector")):
    """ 
    A class that acts as a closure to enhance a graph with XY coordinates 
    """
    def __call__(self, node):
        proj = self.projector.map(node.lat, node.lon)
        node.x = proj.x
        node.y = proj.y
        return node

def path_length(graph, nodes, distance_fn=lambda e: e.distance):
    """ 
    returns the length of a path in the graph given a node list 
    """
    s = 0
    while len(nodes) > 1:
        distances = filter(lambda x: x.to == nodes[1],
                           graph.get_edges(nodes[0]))
        if len(distances) == 0:
            print "%i and %i not connected!" %(nodes[0], nodes[1])
        s += distance_fn(distances[0])
        nodes = nodes[1:]
    return s
