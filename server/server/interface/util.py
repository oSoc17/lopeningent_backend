
from collections import namedtuple

import server.static.data as data
from server.logic.distance.util import get_closest_edge


class SerialNode(namedtuple('SerialNode', 'id mapid lat lon connections')):
    pass


class SerialConn(namedtuple('SerialConn', 'distance node')):
    pass


def serialize_node(graph, index):
    """ Transforms a node index into node information

        Keyword args:
        graph -- the current graph
        index -- the node index
    """
    connlist = [SerialConn(val.distance, node_id)
                for node_id, val in graph.get_conn_idval(index)]
    node = graph.get(index)
    return SerialNode(node.id, node.mapid, node.lat, node.lon, connlist)


def get_edge_tuple(_, lat, lon):
    reallat = float(lat)
    reallon = float(lon)
    location = data.PROJECTOR.map(reallat, reallon)
    return get_closest_edge(location, data.GRAPH, data.GRID)
