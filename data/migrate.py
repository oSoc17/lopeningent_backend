#!/usr/bin/env python
from time import time
from xml.etree.cElementTree import iterparse

"""
1. Load both OSM points and POI points into a grid datastructure.
2. Use R-Tree calculate intersections quickly.
3. Generate Nodes & Edges from this information
4. Write this information to the database
"""

OSM_FILE = "ghent.osm"


class Node:

    def __init__(self, lat, lon):
        self.coord = (lat, lon)


class Edge:

    def __init__(self, osm_id):
        self.osm_id = osm_id
        self.nodes = list()


def load_edges():
    nodes = dict()
    edges = list()

    for event, elem in iterparse(OSM_FILE, events=("start", "end")):
        if event == "start":
            if elem.tag == "node":
                curr_id = int(elem.attrib["id"])
                lat = float(elem.attrib["lat"])
                lon = float(elem.attrib["lon"])
                curr_elem=Node(lat, lon)
            elif elem.tag == "way":
                curr_elem=Edge(int(elem.attrib["id"]))
            elif elem.tag == "nd":
                curr_elem.nodes.append(elem.attrib["ref"])

        elif event == "end":
            if elem.tag == "node":
                nodes[curr_id] = curr_elem
            elif elem.tag == "way":
                edges.append(curr_elem)

    def assign_nodes(edge):
        node_ids=list(edge.nodes)
        edge.nodes=[nodes[int(i)] for i in node_ids]
        return edge

    return map(assign_nodes, edges)

if __name__ == "__main__":
    print "Migration started"
    start=time()
    edges=load_edges()
    end=time()
    print "Migration finished (took {:.5}s)".format(end - start)
