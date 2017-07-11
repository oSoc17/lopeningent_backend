#!/usr/bin/env python
from xml.etree.cElementTree import iterparse
from rtree import index
from time import time
from os import listdir
from json import loads


class Edge:

    def __init__(self, osm_id):
        self.osm_id = osm_id
        self.nodes = list()
        self.attributes = list()

    def get_bounds(self):
        # Get all latitudes and longtitudes inside the edge
        lats = [node[0] for node in self.nodes]
        longs = [node[1] for node in self.nodes]

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

    def assign_nodes(edge):
        node_ids = list(edge.nodes)
        edge.nodes = [nodes[int(i)] for i in node_ids]
        return edge

    return nodes, map(assign_nodes, edges)


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
    def extract_coords(element):
        return {"coord": (element["lat"], element["lon"])}

    for pset in pois:
        # Remove 'null' entries
        pset["elements"] = filter(lambda elem: elem != None, pset["elements"])
        # Only use 'lat' & 'lon'
        pset["elements"] = map(extract_coords, pset["elements"])

    return pois


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
                if pset["name"] not in edge_dict[edge_id].attributes:
                    edge_dict[edge_id].attributes.append(pset["name"])
                else:
                    break

    return edge_dict.values()


def db_write(nodes, edges, pois):
    pass


if __name__ == "__main__":
    start = time()
    nodes, edges = load_osm("ghent.osm")
    pois = load_pois("poisets")
    edges = map_pois(edges, pois)
    db_write(nodes, edges, pois)
    end = time()
    print "Migration took {:.3}s".format(end - start)
