from psycopg2 import pool, InterfaceError
from model.edge import Edge
from model.node import Node
from model.user import User
from logic.graph.util import distance
from time import time
from multiprocessing import Process
import threading, re, config, logging

POOL = pool.ThreadedConnectionPool(1, 20, config.DB_CONN)

def cast_into_point(value):
    match = re.match(r"\(([^)]+),([^)]+)\)", value)

    if match:
        return (float(match.group(1)), float(match.group(2)))

    raise InterfaceError("bad point representation: %r" % value)

def get_nodes(nodes):
    conn = POOL.getconn(key="node")
    cursor = conn.cursor()

    cursor.execute("SELECT nid, coord FROM lopeningent.nodes;")

    for node in cursor.fetchall():
        nid, coord = node
        nodes[nid] = cast_into_point(coord)

    cursor.close()
    POOL.putconn(conn, key="node")


def get_edges(edges):
    conn = POOL.getconn(key="edge")
    cursor = conn.cursor()

    cursor.execute("SELECT eid, rating, tags, from_node, to_node FROM lopeningent.edges;")

    for edge in cursor.fetchall():
        eid, rating, tags, from_node, to_node = edge
        edges[eid] = (rating, tags, from_node, to_node)

    cursor.close()
    POOL.putconn(conn, key="edge")

def get_route_poi(node_list):
    conn = POOL.getconn(key="poi_coords")
    cursor = conn.cursor()
    poi_id =list()
    for node in node_list:
        cursor.execute("SELECT poi_id FROM lopeningent.nodes WHERE nid = %s;",(str(node),))
        pid = cursor.fetchone()
        poi_id.append(pid[0])

    poi_coords = [item for sublist in poi_id for item in sublist]

    cursor.close()
    POOL.putconn(conn, key="poi_coords")
    return poi_coords

def get_graph_data():
    start_time = time()

    # Database variables
    nodes = dict()
    edges = dict()

    # Result variables
    edgelist = list()
    nodelist = list()

    get_nodes(nodes)
    get_edges(edges)

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
    cursor.execute("SELECT uid,avg_speed, avg_heartrate, avg_distance,tot_distance,tot_duration,avg_duration,runs,edit_time FROM lopeningent.users WHERE uid= %s LIMIT 1;",(str(uid),))

    for user in cursor.fetchall():
        uid,avg_speed, avg_heartrate, avg_distance, tot_distance, tot_duration, avg_duration, runs,edit_time  = user
        userFound = User(uid,avg_speed, avg_heartrate, avg_distance, tot_distance, tot_duration, avg_duration, runs,edit_time )
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
                           runs          = (%(runs)s),
                           edit_time     = (%(edit_time)s)
                           WHERE uid=%(uid)s;
                           UPDATE lopeningent.users
                           SET
                           avg_speed     = (%(avg_speed)s),
                           avg_heartrate = (%(avg_heartrate)s),
                           avg_distance  = (%(avg_distance)s),
                           tot_distance  = (%(tot_distance)s),
                           tot_duration  = (%(tot_duration)s),
                           avg_duration  = (%(avg_duration)s),
                           runs          = (%(runs)s),
                           edit_time     = (%(edit_time)s)
                           WHERE uid=%(uid)s;
                           INSERT INTO lopeningent.users
                           (uid,avg_speed, avg_heartrate, avg_distance,tot_distance,tot_duration,avg_duration,runs,edit_time)
                           SELECT %(uid)s, %(avg_speed)s,%(avg_heartrate)s, %(avg_distance)s, %(tot_distance)s, %(tot_duration)s, %(avg_duration)s, %(runs)s,%(edit_time)s
                           WHERE NOT EXISTS (SELECT 1 FROM lopeningent.users WHERE uid=%(uid)s);
                        """, {'uid': user.uid, 'avg_speed': user.avg_speed, 'avg_heartrate': user.avg_heartrate, 'avg_distance': user.avg_distance, 'tot_distance': user.tot_distance, 'tot_duration': user.tot_duration, 'avg_duration': user.avg_duration, 'runs': user.runs, 'edit_time': user.edit_time })
        logging.debug("STATS DB:" + str(conn.status))

        conn.commit()
        logging.info("inserted/updated users table with id: " + str(user.uid))
        cursor.close()
        POOL.putconn(conn, key="update-stats")
        return True
    except Exception, e:
        logging.error("something went wrong when updating/inserting stats for id: " + str(user.uid))
        cursor.close()
        POOL.putconn(conn, key="update-stats")
        return False


def update_edge_in_db(edge, new_rating):
    conn = POOL.getconn(key="edge-update")
    logging.info("update edge rating in db")
    cursor = conn.cursor()

    cursor.execute(
        """
        SELECT * FROM lopeningent.edges WHERE from_node = %s AND to_node = %s;
        """, (edge.id, edge.to)
    )

    eid, rating, _, _, _ = cursor.fetchone()
    new_rating = (rating + new_rating) / 2    

    cursor.execute(
        """
        UPDATE lopeningent.edges
        SET
        rating = %s
        WHERE eid = %s
        """, (new_rating, eid)
    )

    cursor.close()
    conn.commit()
    POOL.putconn(conn, key="edge-update")


def get_poi_coords(types,route_poi):
    conn = POOL.getconn(key="get-poi-coords")
    cursor = conn.cursor()

    coords = list()


    for type in types:
        cursor.execute(
            """
            SELECT pid, name, description, coord 
            FROM lopeningent.pois WHERE type = %s
            """,
            (type, )
        )

        for row in cursor.fetchall():
            if row[0] in route_poi:
                lat, lon = cast_into_point(row[3])
                coords.append({
                    "name": row[1],
                    "description": row[2],
                    "lat": lat,
                    "lon": lon,
                    "type": type
                })
    
    return coords
    cursor.close()
    POOL.putconn(conn, key="get-poi-coords")

def get_poi_types():
    conn = POOL.getconn(key="get-poi-types")
    cursor = conn.cursor()
    cursor.execute("SELECT DISTINCT type FROM lopeningent.pois;")
    return [row[0] for row in cursor.fetchall()]
    cursor.close()
    POOL.putconn(conn, key="get-poi-types")