from psycopg2 import pool, InterfaceError
from model.edge import Edge
from model.node import Node
from model.user import User
from logic.graph.util import distance
from time import time
import threading, re, config

POOL = pool.ThreadedConnectionPool(1, 20, config.DB_CONN)

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

    cursor.execute("SELECT eid, rating, tags, from_node, to_node FROM lopeningent.edges;")

    for edge in cursor.fetchall():
        eid, rating, tags, from_node, to_node = edge
        edges[eid] = (rating, tags, from_node, to_node)

    cursor.close()
    POOL.putconn(conn, key="edge")


def get_graph_data():
    start_time = time()

    # Database variables
    nodes = dict()
    edges = dict()

    # Result variables
    edgelist = list()
    nodelist = list()

    # TODO: Run these in parallel (in a way that doesn't conflict with Django)
    _get_nodes(nodes)
    _get_edges(edges)

    # Wrap every node into a model class and hand it off to the result list
    for nid, coord in nodes.iteritems():
        lat, lon = coord
        nodelist.append(Node(nid, lat, lon))

    # wrap the edges into a model class and save them to the edgelist
    for edata in edges.values():
        rating, tags, start, end = edata
        dist = distance(nodes[start], nodes[end])
        new_edge = Edge(start, dist, 0.0, 1.0, end)
        new_edge.set_modifier_data(rating, tags)
        edgelist.append(new_edge)

    end_time = time()
    return nodelist, edgelist

def get_stats_user(uid):
    conn = POOL.getconn(key="get-stats")
    cursor = conn.cursor()
    userFound = []
    cursor.execute("SELECT uid,avg_speed, avg_heartrate, avg_distance,tot_distance,tot_duration,avg_duration,runs FROM lopeningent.users WHERE uid= %s LIMIT 1;",(int(uid),))

    for user in cursor.fetchall():
        uid,avg_speed, avg_heartrate, avg_distance, tot_distance, tot_duration, avg_duration, runs = user
        userFound = User(uid,avg_speed, avg_heartrate, avg_distance, tot_distance, tot_duration, avg_duration, runs)
    try:
        print  userFound.toJSON()
        cursor.close()
        return  userFound.toJSON()
    except AttributeError:
        cursor.close()
        return  None

    print userFound.toJSON()
    cursor.close()
    POOL.putconn(conn, key="get-stats")
    return userFound.toJSON()

def update_stats_user(user):
    conn = POOL.getconn(key="update-stats")
    try:
        cursor = conn.cursor()
        cursor.execute("""
                        UPDATE lopeningent.users
                        SET
                        avg_speed     = (%(avg_speed)s),
                        avg_heartrate = (%(avg_heartrate)s),
                        avg_distance  = (%(avg_distance)s),
                        tot_distance  = (%(tot_distance)s),
                        tot_duration  = (%(tot_duration)s),
                        avg_duration  = (%(avg_duration)s),
                        runs          = (%(runs)s)
                        WHERE uid=%(uid)s;
                        UPDATE lopeningent.users
                        SET
                        avg_speed     = (%(avg_speed)s),
                        avg_heartrate = (%(avg_heartrate)s),
                        avg_distance  = (%(avg_distance)s),
                        tot_distance  = (%(tot_distance)s),
                        tot_duration  = (%(tot_duration)s),
                        avg_duration  = (%(avg_duration)s),
                        runs          = (%(runs)s)
                        WHERE uid=%(uid)s;
                        INSERT INTO lopeningent.users
                        (uid,avg_speed, avg_heartrate, avg_distance,tot_distance,tot_duration,avg_duration,runs)
                        SELECT %(uid)s, %(avg_speed)s,%(avg_heartrate)s, %(avg_distance)s, %(tot_distance)s, %(tot_duration)s, %(avg_duration)s, %(runs)s
                        WHERE NOT EXISTS (SELECT 1 FROM lopeningent.users WHERE uid=%(uid)s);


                        ;""",{'uid' : user.uid,'avg_speed':user.avg_speed,'avg_heartrate': user.avg_heartrate,'avg_distance': user.avg_distance,'tot_distance': user.tot_distance,'tot_duration':None,'avg_duration':None,'runs' : user.runs})
        conn.commit()
        print "inserted/updated users table with id: " + str(user.uid)

        cursor.close()
        POOL.putconn(conn, key="update-stats")
        return True
    except Exception, e:
        print "something went wrong when updating/inserting stats for id: " + str(user.uid)
        cursor.close()
        POOL.putconn(conn, key="update-stats")
        return False



def update_edge_in_db(edge):
    conn = POOL.getconn(key="edge-update")
    cursor = conn.cursor()

    cursor.execute(
        """
        UPDATE lopeningent.edges
        SET
        rating = %s
        WHERE eid = %s
        """,
        (edge._rating, edge.id)
    )

    cursor.close()
    conn.commit()
    POOL.putconn(conn, key="edge-update")