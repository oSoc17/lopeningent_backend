#!/usr/bin/env python
from xml.etree.cElementTree import iterparse
from time import time
from os import listdir
from json import loads

"""
1. Load both OSM points and POI points into a grid datastructure.
2. Use R-Tree calculate intersections quickly.
3. Generate Nodes & Edges from this information
4. Write this information to the database
"""


class Node:

    def __init__(self, lat, lon):
        self.coord = (lat, lon)


class Edge:

    def __init__(self, osm_id):
        self.osm_id = osm_id
        self.nodes = list()


def load_edges(osm_file):
    nodes = dict()
    edges = list()

    for event, elem in iterparse(osm_file, events=("start", "end")):
        if event == "start":
            if elem.tag == "node":
                curr_id = int(elem.attrib["id"])
                lat = float(elem.attrib["lat"])
                lon = float(elem.attrib["lon"])
                curr_elem = Node(lat, lon)
            elif elem.tag == "way":
                curr_elem = Edge(int(elem.attrib["id"]))
            elif elem.tag == "nd":
                curr_elem.nodes.append(elem.attrib["ref"])

        elif event == "end":
            if elem.tag == "node":
                nodes[curr_id] = curr_elem
            elif elem.tag == "way":
                edges.append(curr_elem)

    def assign_nodes(edge):
        node_ids = list(edge.nodes)
        edge.nodes = [nodes[int(i)] for i in node_ids]
        return edge

    return map(assign_nodes, edges)


def load_pois(poi_dir):
    """
    Converts each poi_set in the poi_sets/ directory 
    into a list of dictionaries containing 
    the latitudes and longtitudes of each point.

    :return: poi_data (list)
    """
    pois = list()

    # load all the poi sets from JSON into a dictionary.
    for filename in listdir(poi_dir):
        with open(poi_dir + "/" + filename, 'r') as file:
            pois = loads(file.read())

    # helper function to extract coords out of an element
    def extract_coords(element):
        return {
            "lat": element["lat"],
            "lon": element["lon"]
        }

    # Remove 'null' entries
    pois["elements"] = filter(lambda elem: elem != None, pois["elements"])
    # Only use 'lat' & 'lon'
    pois["elements"] = map(extract_coords, pois["elements"])

    return pois

if __name__ == "__main__":
    print "Migration started"
    start = time()
    edges = load_edges("ghent.osm")
    print "Loaded edges..."
    pois = load_pois("poisets")
    print "Loaded points of interest..."
    end = time()
    print "Migration finished (took {:.5}s)".format(end - start)
