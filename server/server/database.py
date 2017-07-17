from psycopg2 import pool, InterfaceError
from model.edge import Edge
from model.node import Node
from time import time
import threading, re, config

POOL = pool.ThreadedConnectionPool(1, 20, config.DB_CONN)

def _get_edge_nodes(edge_nodes):
    conn = POOL.getconn(key="rel")
    cursor = conn.cursor()

    cursor.execute("SELECT eid, nid FROM lopeningent.edge_nodes;")

    for relation in cursor.fetchall():
        eid, nid = relation

        if eid not in edge_nodes:
            edge_nodes[eid] = list()

        edge_nodes[eid].append(nid)

    cursor.close()
    POOL.putconn(conn, key="rel")


def _get_nodes(nodes):
    conn = POOL.getconn(key="node")
    cursor = conn.cursor()

    cursor.execute("SELECT nid, coord FROM lopeningent.nodes;")

    def cast_into_point(value):
        match = re.match(r"\(([^)]+),([^)]+)\)", value)

        if match:
            return (float(match.group(1)), float(match.group(2)))

        raise InterfaceError("bad point representation: %r" % value)

    for node in cursor.fetchall():
        nid, coord = node
        nodes[nid] = cast_into_point(coord)

    cursor.close()
    POOL.putconn(conn, key="node")


def _get_edges(edges):
    conn = POOL.getconn(key="edge")
    cursor = conn.cursor()

    cursor.execute("SELECT eid, rating, tags FROM lopeningent.edges;")

    for edge in cursor.fetchall():
        eid, rating, tags = edge
        edges[eid] = (rating, tags)

    cursor.close()
    POOL.putconn(conn, key="edge")


def get_graph_data():
    start_time = time()

    # Database variables
    relations = dict()
    nodes = dict()
    edges = dict()

    # Result variables
    edgelist = list()
    nodelist = list()

    # TODO: Run these in parallel (in a way that doesn't conflict with Django)
    _get_edge_nodes(relations)
    _get_nodes(nodes)
    _get_edges(edges)

    # Wrap every node into a model class and hand it off to the result list
    for nid, coord in nodes.iteritems():
        lat, lon = coord
        nodelist.append(Node(nid, lat, lon))

    # Go through all the edges, create new edges between two nodes instead
    # of multiple nodes per edge, wrap them into a model class and pass
    # them off to the result list.
    for eid, nids in relations.iteritems():
        for start, end in zip(nids, nids[1:]):
            rating, tags = edges[eid]
            edge = Edge(start, 0.0, 0.0, 1.0, end)
            edge.set_modifier_data(rating, tags)
            edgelist.append(edge)

    end_time = time()
    print "merging the dataset took {} seconds".format(end_time - start_time)
    return nodelist, edgelist