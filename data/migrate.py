#!/usr/bin/env python
import psycopg2
from xml.etree.cElementTree import iterparse
from rtree import index
from time import time
from os import listdir
from json import loads
import migrate_config as C


class Edge:

    def __init__(self, osm_id):
        self.osm_id = osm_id
        self.nodes = list()
        self.tags = list()

    def get_bounds(self):
        # Get all latitudes and longtitudes inside the edge
        lats = [node["coord"][0] for node in self.nodes]
        longs = [node["coord"][1] for node in self.nodes]

        # Get the smallest and largest coordinate points
        min_coord = (min(lats), min(longs))
        max_coord = (max(lats), max(longs))

        return (min_coord, max_coord)

    def into_box(self, radius=0.001):
        min_coord, max_coord = self.get_bounds()

        # Helper function to apply some space around the edge
        def apply_radius(coord, radius):
            lat, lon = coord
            return (round(lat + radius, 7), round(lon + radius, 7))

        # Create a bounding box around an edge
        min_coord = apply_radius(min_coord, -radius)
        max_coord = apply_radius(min_coord, radius)
        left, bottom = min_coord
        right, top = max_coord

        # This is the order required by rtree
        return (left, bottom, right, top)


def load_osm(osm_file):
    """
    loads all edges and nodes from the .osm (XML) file 
    and wraps them into objects.

    :param osm_file: filename of the .osm file containing the map data.
    :return: edges (list), nodes (dict)
    """
    nodes = dict()
    edges = list()

    for event, elem in iterparse(osm_file, events=("start", "end")):
        # Whenever the iterator encounters an opening tag
        if event == "start":
            if elem.tag == "node":
                curr_id = int(elem.attrib["id"])
                lat = float(elem.attrib["lat"])
                lon = float(elem.attrib["lon"])
                curr_elem = (lat, lon)
            elif elem.tag == "way":
                curr_elem = Edge(int(elem.attrib["id"]))
            elif elem.tag == "nd":
                curr_elem.nodes.append(elem.attrib["ref"])

        # Whenever the iterator encounters a closing tag
        elif event == "end":
            if elem.tag == "node":
                nodes[curr_id] = curr_elem
            elif elem.tag == "way":
                edges.append(curr_elem)

    return nodes, edges


def load_pois(poi_dir):
    """
    Converts each poi_set in the poi_sets/ directory 
    into a list of dictionaries containing 
    the latitudes and longtitudes of each point.

    :param poi_dir: directory name containing poi files in JSON. (string)
    :return: pois (list of dict of poi sets)
    """
    pois = list()

    # load all the poi sets from JSON into a dictionary.
    for filename in listdir(poi_dir):
        with open(poi_dir + "/" + filename, 'r') as file:
            pois.append(loads(file.read()))

    # helper function to extract coords out of an element
    def extract_relevant(element):
        name = "" if "name" not in element else element["name"]    
        descr = "" if "description" not in element else element["description"]

        return {
            "name": name,
            "description": descr,
            "coord": (element["lat"], element["lon"])
        }

    for pset in pois:
        # Remove 'null' entries
        pset["elements"] = filter(lambda elem: elem != None, pset["elements"])
        # Only use 'lat' & 'lon'
        pset["elements"] = map(extract_relevant, pset["elements"])

    return pois


def update_edge_nodes(edges, nodes, osm_nid_dict):
    def assign_nodes(edge):
        osm_ids = list(edge.nodes)
        edge.nodes = list()
        for osm_id in osm_ids:
            nid = osm_nid_dict[int(osm_id)]
            edge.nodes.append({"nid": nid, "coord": nodes[int(osm_id)]})
        return edge

    return map(assign_nodes, edges)


def map_pois(edges, pois):
    """
    An R-tree is a datastructure which allows us to 
    quickly find the intersection between 
    two polygons on a spatial plane. In this case
    we're trying to find all of the edges 
    that intersect with POIs. After that, 
    we update the edges with this info.

    https://en.wikipedia.org/wiki/R-tree

    :param edges: edges which have node data included (list of Edge)
    :param poi: all POI sets (list of POI sets)
    """
    # Open up an rtree
    idx = index.Rtree()
    edge_dict = dict()

    # Transform edges into boxes and put them inside the rtree
    for i, edge in enumerate(edges):
        edge_dict[i] = edge
        idx.insert(i, edge.into_box())

    # Helper function to turn a poi into a box
    def poi_into_box(poi, radius=0.001):
        lat, lon = poi["coord"]
        left = round(lat - radius, 7)
        bottom = round(lon - radius, 7)
        right = round(lat + radius, 7)
        top = round(lon + radius, 7)
        return (left, bottom, right, top)

    # Helper function to find edge/poi intersections
    def find_intersects(poi):
        poi = poi_into_box(poi)
        return [i for i in idx.intersection(poi)]

    # Go through all the POI sets and check for
    # intersections with the edges inside the rtree.
    # If it's intersecting, add an attribute
    # with the name of the POI set.
    for pset in pois:
        for element in pset["elements"]:
            for edge_id in find_intersects(element):
                if pset["name"] not in edge_dict[edge_id].tags:
                    edge_dict[edge_id].tags.append(pset["name"])
                else:
                    break

    return edge_dict.values()


def db_connect(connstr):
    return psycopg2.connect(connstr)


def db_truncate(connection):
    cursor = connection.cursor()
    cursor.execute(
        """
        TRUNCATE lopeningent.edges, lopeningent.nodes, lopeningent.pois 
            RESTART IDENTITY CASCADE
        """
    )
    cursor.close()


def db_write_nodes(connection, nodes):
    cursor = connection.cursor()
    osm_nid_dict = dict()

    def convert_node(node):
        return "({}, {})".format(node[0], node[1])

    for osm_id, node in nodes.iteritems():
        cursor.execute(
            """
            INSERT INTO lopeningent.nodes (coord) 
                VALUES (%s) RETURNING nid
            """
            , (convert_node(node), )
        )
        osm_nid_dict[osm_id] = cursor.fetchone()[0]

    cursor.close()
    return osm_nid_dict


def db_write_edges(connection, edges):
    cursor = connection.cursor()

    def list_into_pg(list):
        list = str(map(str, list))
        return list.replace('[', '{').replace(']', '}').replace('\'', '\"')

    one_to_one_edges = list()

    for edge in edges:
        edge.nodes = map(lambda node: node['nid'], edge.nodes)
        for start, end in zip(edge.nodes, edge.nodes[1:]):
            one_to_one_edges.append((start, end, edge.tags))
            one_to_one_edges.append((end, start, edge.tags))

    for e in one_to_one_edges:
        fr, to, tags = e
        cursor.execute(
            """
            INSERT INTO lopeningent.edges (rating, tags, from_node, to_node)
                VALUES (%s, %s, %s, %s)
            """
            , (2.5, list_into_pg(tags), fr, to)
        )

    cursor.close()


def db_write_pois(connection, pois):
    cursor = connection.cursor()

    def convert_poi_coord(poi):
        return "({}, {})".format(poi["coord"][0], poi["coord"][1])

    for pset in pois:
        for poi in pset["elements"]:
            cursor.execute(
                """
                INSERT INTO lopeningent.pois (name, description, coord, type)
                    VALUES(%s, %s, %s, %s)
                """
                , (poi["name"], poi["description"], 
                    convert_poi_coord(poi), pset["name"])
            )

    cursor.close()


def db_close(connection):
    connection.commit()
    connection.close()


if __name__ == "__main__":
    start = time()
    print "Please be patient, this could take a while..."

    nodes, edges = load_osm(C.OSM_FILE)
    pois = load_pois(C.POI_DIR)
    end = time()
    print "Loaded coordinates into memory... ({})".format(end - start)

    conn = db_connect(C.DB_CONN)
    db_truncate(conn)
    end = time()
    print "Cleared database... ({})".format(end - start)

    osm_nid_dict = db_write_nodes(conn, nodes)

    edges = map_pois(update_edge_nodes(edges, nodes, osm_nid_dict), pois)
    end = time()
    print "Mapped POIs to edges... ({})".format(end - start)

    db_write_edges(conn, edges)
    db_write_pois(conn, pois)

    db_close(conn)
    end = time()
    "Wrote changes to the database... ({})".format(end -start)

    end = time()
    print "Migration finished ({})".format(end - start)
