from server.logic.graph.graph import Graph
from server.config import HIGHWAY_WHITELIST
import json
from collections import namedtuple
from server.logic.graph.util import distance
from server.logic.database.edge_database import EdgeDatabase
DATABASE = EdgeDatabase()


class Coordinate(namedtuple('node', 'lat lon')):
    pass


class Vertex(namedtuple('node', 'id mapid lat lon x y meta')):
    def into_coordinate(self):
        return Coordinate(self.lat, self.lon)


class Edge(namedtuple('edge', 'id distance highway rating_sum rating_count water park to')):
    pass


def load(dirname):
    """Factory method for creating a graph.

    Function args:
    dirname -- name of a directory containing files named "nodes.json" and "ways.json"
    """
    with open(dirname + "/nodes.json", "r") as f:
        jsonobj = json.loads(f.read())
        # remove duplicates
        nodelist_spatials = {
            (rawnode["lat"], rawnode["lon"]): rawnode["id"] for rawnode in jsonobj["elements"]}
        enumerator = zip(sorted(nodelist_spatials.items(), key=lambda x: x[1]), xrange(
            len(nodelist_spatials.items())))
        node = {val: Vertex(i, nodelist_spatials[key], key[0], key[1], 0.0, 0.0, []) for (
            key, val), i in enumerator}
        enumerator = zip(jsonobj["elements"], xrange(len(jsonobj["elements"])))
        nodelist = {rawnode["id"]: node[nodelist_spatials[(
            rawnode["lat"], rawnode["lon"])]]._replace(meta=[rawnode["water"], rawnode["park"]]) for rawnode, i in enumerator}
        jsonobj = None

    waylist = []
    edge_dictionary = DATABASE.get_all_edges()
    with open(dirname + "/ways.json", "r") as f:
        jsonobj = json.loads(f.read())
        for way in jsonobj["elements"]:
            if way["tags"]["highway"] == "motorway":
                continue
            for start, end in zip(way["nodes"], way["nodes"][1:]):
                dist = distance(nodelist[start], nodelist[end])
                start_id = nodelist[start].id
                end_id = nodelist[end].id
                (water, park) = map(lambda x, y: 1 if x == "yes" and y == "yes" else 0,
                                    nodelist[start].meta, nodelist[end].meta)
                (r_sum, r_count, r_avg) = edge_dictionary.get((start_id, end_id), (0.0, 0, 0.0))
                waylist.append((start_id,
                                Edge(start_id,
                                     dist,
                                     HIGHWAY_WHITELIST.get(way['tags']['highway'], HIGHWAY_WHITELIST['default']),
                                     r_sum,
                                     r_count,
                                     water,
                                     park,
                                     end_id),
                                end_id))
                waylist.append((end_id,
                                Edge(end_id,
                                     dist,
                                     HIGHWAY_WHITELIST.get(way['tags']['highway'], HIGHWAY_WHITELIST['default']),
                                     r_sum,
                                     r_count,
                                     water,
                                     park,
                                     start_id),
                                start_id))
    print("Graph constructed")
    return Graph([i for _, i in nodelist.items()], waylist)


class VertexXY(namedtuple('node_xy', 'id mapid lat lon x y')):
    def into_coordinate(self):
        return Coordinate(self.lat, self.lon)


def project(graph, projector):
    """ Creates an enhanced graph with xy coordinates from a graph """
    return graph.map_graph(Mapper(projector), lambda x: x)


class Mapper(namedtuple("Mapper", "projector")):
    """ A class that acts as a closure to enhance a graph with XY coordinates """
    def __call__(self, node):
        proj = self.projector.map(node)
        return VertexXY(node.id, node.mapid, node.lat, node.lon, proj.x, proj.y)


def path_length(graph, nodes, distance_fn=lambda e: e.distance):
    """ returns the length of a path in the graph given a node list """
    s = 0
    while len(nodes) > 1:
        distances = filter(lambda x: x.to == nodes[1],
                           graph.get_edges(nodes[0]))
        if len(distances) == 0:
            print "%i and %i not connected!" %(nodes[0], nodes[1])
        s += distance_fn(distances[0])
        nodes = nodes[1:]
    return s
