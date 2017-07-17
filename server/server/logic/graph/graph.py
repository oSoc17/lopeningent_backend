# Implementation of the class AbstractGraph
# This implementation is templated

from collections import namedtuple

from cffi import FFI

from server.model.edge import Edge
from server.model.node import Node
from server.logic.graph.util import parse

ffi = FFI()

lib = ffi.dlopen("target/release/libgraphing.so")
print('Loaded lib {0}'.format(lib))
with open("include/graphing.h") as f:
    ffi.cdef(parse(f.read()))

stop = 2**64 - 1


class DijkstraIterator(object):
    """
    Iterator that yields the edges and nodes of a graph, 
    sorted by distance to the origin.
    This class should be instantiated by one of the methods of Graph.
    """
    def __iter__(self):
        return self

    def next(self):
        res = lib.graph_dijkstra_next(self.dijkstra)
        if res.id == stop:
            raise StopIteration
        return (res.id, (res.size_value, res.actual_value))

    def root(self, index):
        """
        Yields the shortest path between node index and the starting node.
        """
        root = lib.graph_dijkstra_root(self.dijkstra, index)
        res = []
        while True:
            n = lib.graph_root_next(root)
            if n == stop:
                lib.graph_root_delete(root)
                return res
            res.append(n)

    def filter(self, rod):
        """
        Creates a new iterator that only yields when 
        the destination node is in rod.
        """
        rods = ffi.new("size_t[]", rod)
        self.dijkstra = lib.graph_dijkstra_filter(self.dijkstra, rods, len(rod))
        return self

    def choose(self, config):
        """
        Creates a new iterator that yields a single random node.
        """
        conf = self.gen_config(config)
        self.dijkstra = lib.graph_dijkstra_choose(self.dijkstra, conf)
        return self

    def gen_config(self, config):
        """
        Generate configuration.
        """
        return ffi.new("Configuration*", 
            (config.measure_length, config.max_length, config.min_length))

class Graph:
    """
        Main class for holding information about roads and crossroads.
        Most of the functionality is contained in the .so file.
    """

    def __init__(self, nodelist, edgelist):
        """ 
        Create a new graph given nodes and edges.
        """

        self.lib = lib
        self.largest = max(node.id for node in nodelist)

        nodes = [node.into_c() for node in nodelist]
        edges = [edge.into_c() for edge in edgelist]
        self.graph = lib.graph_new(
            ffi.new("Node[]", nodes), len(nodes),
            ffi.new("Edge[]", edges), len(edges)
        )

    def __del__(self):
        self.lib.graph_delete(self.graph)

    def _c_edge_into_python(self, c_edge):
        return Edge(c_edge.id, c_edge.distance, c_edge.modifier, c_edge.poison, c_edge.to)

    def _c_node_into_python(self, c_node):
        return Node(c_node.id, c_node.lat, c_node.lon, c_node.x, c_node.y)

    def get(self, index):
        return self._c_node_into_python(lib.graph_get(self.graph, index))

    def get_conn_idval(self, index):
        """ 
        Returns a list of connections given the index
        """
        class ConnIdVal(object):
            def __init__(self, graph, index):
                self.connidval = lib.graph_conn_idval_new(graph, index)

            def __iter__(self):
                return self

            def next(self):
                res = lib.graph_conn_idval_next(self.connidval)
                if res.e == ffi.NULL:
                    raise StopIteration
                return res

            def __del__(self):
                lib.graph_conn_idval_delete(self.connidval)

        return [self._c_edge_into_python(i.e) for i in ConnIdVal(self.graph, index)]

    def get_edges(self, index):
        class Edges(object):
            def __init__(self, graph, index):
                self.edges = lib.graph_edges_new(graph, index)

            def __iter__(self):
                return self

            def next(self):
                res = lib.graph_edges_next(self.edges)
                if res == ffi.NULL:
                    raise StopIteration
                return res

            def __del__(self):
                lib.graph_edges_delete(self.edges)

        return [self._c_edge_into_python(e) for e in Edges(self.graph, index)]

    def get_connids(self, index):
        """ Returns a list of node indices the node itself is connected to. """
        class ConnIds(object):
            def __init__(self, graph, index):
                self.connids = lib.graph_connids_new(graph, index)

            def __iter__(self):
                return self

            def next(self):
                res = lib.graph_connids_next(self.connids)
                if res == stop:
                    raise StopIteration
                return res

            def __del__(self):
                lib.graph_connids_delete(self.connids)

        return [int(i) for i in ConnIds(self.graph, index)]

    def list_ids(self):
        """ Returns a list of all indices in the graph. """
        class ListIds(object):
            def __init__(self, graph):
                self.listids = lib.graph_listids_new(graph)

            def __iter__(self):
                return self

            def next(self):
                res = lib.graph_listids_next(self.listids)
                if res == stop:
                    raise StopIteration
                return res

            def __del__(self):
                lib.graph_listids_delete(self.listids)

        return (i for i in ListIds(self.graph))

    def contains(self, index):
        """ Checks if a node index is present """
        if index < 0:
            return False
        return lib.graph_contains(self.graph, index)

    def iter_nodes(self):
        """ Iterates over all nodes in a graph, in (id, node_data) format.

        Useful for transformation into a new graph """
        return (self.get(node_id) for node_id in self.list_ids())

    def iter_edges(self):
        """ Iterates over all edges in a graph, in (id, edge_data, to) format.

        Useful for transformation into a new graph """
        return (
            conn for id in self.list_ids() for conn in self.get_conn_idval(id)
        )

    def map_graph(self, node_fn, edge_fn):
        """ 
        Creates a new graph, which has the same structure, 
        but every node and edge data field has 
        the function node_fn or edge_fn applied to it.
        """
        nodelist = [node_fn(node) for node in self.iter_nodes()]
        edgelist = [edge_fn(edge) for edge in self.iter_edges()]
        return Graph(nodelist, edgelist)

    def generate_dijkstra(self, start_node, config):
        class DijkstraIteratorPath(DijkstraIterator):
            def __init__(self, graph, start_node, config):
                self.graph = graph
                configuration = self.gen_config(config)
                self.dijkstra = lib.graph_graph_generate(graph, start_node, configuration)

            def __del__(self):
                lib.graph_dijkstra_delete(self.dijkstra)

        return DijkstraIteratorPath(self.graph, start_node, config)

    def add_rating(self, start_node, end_node, rating):
        #lib.graph_update_rating(self.graph, start_node, end_node, rating)
        pass
