from psycopg2 import pool, InterfaceError
from model.edge import Edge
from model.node import Node
from time import time
import threading, re, config

POOL = pool.ThreadedConnectionPool(1, 20, config.DB_CONN)


def _get_edge_nodes(edge_nodes):
    conn = POOL.getconn()
    cursor = conn.cursor()

    cursor.execute("SELECT eid, nid FROM lopeningent.edge_nodes;")

    for relation in cursor.fetchall():
        eid, nid = relation

        if eid not in edge_nodes:
            edge_nodes[eid] = list()

        edge_nodes[eid].append(nid)

    cursor.close()
    POOL.putconn(conn)


def _get_nodes(nodes):
    conn = POOL.getconn()
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
    POOL.putconn(conn)


def _get_edges(edges):
    conn = POOL.getconn()
    cursor = conn.cursor()

    cursor.execute("SELECT eid, rating, tags FROM lopeningent.edges;")

    for edge in cursor.fetchall():
        eid, rating, tags = edge
        edges[eid] = (rating, tags)

    cursor.close()


def get_graph_data():
    start_time = time()

    # Threading variables
    main_thread = threading.currentThread()
    threads = list()

    # Database variables
    relations = dict()
    nodes = dict()
    edges = dict()

    # Result variables
    edgelist = list()
    nodelist = list()

    # We're executing all select statements in parallel 
    threads.append(threading.Thread(target=_get_edge_nodes, args=(relations,)))
    threads.append(threading.Thread(target=_get_nodes, args=(nodes,)))
    threads.append(threading.Thread(target=_get_edges, args=(edges,)))

    # Boot up all the threads
    for thread in threads:
        thread.start()

    # Wait for every thread to finish
    for thread in threading.enumerate():
        if thread is main_thread:
            continue
        thread.join()

    # Wrap every node into a model class and hand it off to the result list
    for nid, coord in nodes.iteritems():
        nodelist.append(Node(nid, coord))

    # Go through all the edges, create new edges between two nodes instead
    # of multiple nodes per edge, wrap them into a model class and pass
    # them off to the result list.
    for eid, nids in relations.iteritems():
        for start, end in zip(nids, nids[1:]):
            rating, tags = edges[eid]
            edgelist.append(Edge(start, end, rating, tags))

    end_time = time()
    print "merging the dataset took {} seconds".format(end_time - start_time)
    return edgelist, nodelist