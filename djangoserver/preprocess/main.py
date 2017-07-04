from xml.etree.cElementTree import iterparse
from data import *
import time
import sys
import gc
import json

import os
import sys
sys.path.append('.')

from shapely import speedups
from shapely.geometry import MultiPoint, Point
from server.logic.grid.interval import Interval
from server.logic.projection.util import Coordinate


# --------------------
# configuration
# --------------------

# Distance that is considered 'close' (in meter).
# This is only valid for a latitude of 51 degrees and even then with about 20% accuracy.
# Extension to general use: transform coordinates. Considered out-of-scope.
close_distance_meter = 10.0

# conversion to degree, only valid for lat=51.
close_distance = close_distance_meter / 90723

path_to_file = "randomdeployment/mapdata.osm"

path_to_output_dir = "dataonsteroids/"  # NEEDS the last '/'

park_tags = {
    "leisure": ["park", "common", "nature_reserve"],
    "landuse": ["farmland", "forest", "grass", "meadow", "orchard", "recreation_ground", "village_green"],
    "natural": ["wood", "grassland"]
}

water_tags = {
    "natural": ["water", "bay", "beach"]
}


# --------------------
# end configuration
# --------------------


# enable Shapely constructor speedups
if speedups.available:
    speedups.enable()


def parse_xml():
    """
    Parses the xml in the file.
    :return: nodes (dict), ways (dict) and polygons (list)
    """
    node_dict = dict()
    way_dict = dict()
    polygon_list = list()

    current_element = None

    for event, elem in iterparse(path_to_file, events=("start", "end")):
        # when the xml tag is opened: we have access to the attributes
        if event == "start":
            if elem.tag == "node":
                current_element = Node(id=int(elem.attrib['id']), lat=float(elem.attrib['lat']),
                                       lon=float(elem.attrib['lon']), water=0, park=0, tags=dict())
            elif elem.tag == "tag":
                current_element.tags[elem.attrib['k']] = elem.attrib['v']
            elif elem.tag == "way":
                current_element = Way(id=int(elem.attrib['id']), nodes=list(), tags=dict())
            elif elem.tag == "nd":
                current_element.nodes.append(int(elem.attrib['ref']))
            elif elem.tag == "relation":
                current_element = Relation(id=int(elem.attrib['id']), members=list(), tags=dict())
            elif elem.tag == "member":
                current_element.members.append((elem.attrib['type'], int(elem.attrib['ref']), elem.attrib['role']))

        # when the xml tag is closed: the whole object should be constructed
        elif event == "end":
            if elem.tag == "node":
                node_dict[current_element.id] = current_element
            elif elem.tag == "way":
                way_dict[current_element.id] = current_element
            elif elem.tag == "relation":
                dic = current_element.tags
                if "type" in dic and dic["type"] == "multipolygon":
                    polygon_list.append(current_element)

            elem.clear()
        else:
            print "Received event that is not start and not end"
            sys.exit(-1)

    return node_dict, way_dict, polygon_list


def get_high_and_water_ways(ways):
    """
    Extracts highways and waterways from all ways. 
    :param ways: All ways as a dict
    :return: highways (list), waterways (list)
    """
    highways = list()
    waterways = list()
    for way_id in ways:
        way = ways[way_id]
        if "highway" in way.tags:
            highways.append(way)
        elif "waterway" in way.tags:
            waterways.append(way)
    return highways, waterways


def get_routable_nodes_and_ways(nodes, highways):
    """
    Extracts the routable nodes and ways from all nodes and highways.
    :param nodes: All nodes as a dict
    :param highways: All ways that have the 'highway' tag as list
    :return: routable_nodes (dict), routable_ways (dict)
    """
    useful_nodes = dict()
    useful_ways = dict()

    for way in highways:
        useful = True
        for node_id in way.nodes:
            node = nodes.get(node_id, None)
            if node is None:
                useful = False
            else:
                if node_id not in useful_nodes:
                    useful_nodes[node_id] = node
        if useful:
            useful_ways[way.id] = way

    return useful_nodes, useful_ways


def extract_outerway_from_polygon(polygon, all_ways):
    """
    Extracts the outer border from a polygon.
    :param polygons: Polygon to extract the outer way from.
    :param all_ways: All ways of the graph.
    :return: A way that makes up the whole outer border. Not in right order. None if an error happens.
    """
    nodes = list()
    for member in polygon.members:
        if member[2] == "outer":
            old_way = all_ways.get(member[1], None)
            if old_way is None:
                return None  # when we can't find the way in the graph, skip this polygon
            nodes.extend(old_way.nodes)
    return Way(id=1, nodes=nodes, tags={})


def build_poly(way, nodes):
    """
    Builds a Shapely.Polygon from a way.
    :param way: Way to convert.
    :param nodes: All nodes in the graph.
    :return: Polygon of the way.
    """
    nodes_in_way = [nodes[id] for id in way.nodes]
    coords_in_way = [(node.lat, node.lon) for node in nodes_in_way]
    poly = MultiPoint(coords_in_way).convex_hull
    if poly.is_valid:
        return poly
    else:
        return None


def get_parks(nodes, ways, polygons):
    """
    Extracts all parks from all ways and polygons. Returns them as Shapely Polygons.
    :param nodes: All nodes in the graph (dict)
    :param ways: All ways in the graph (dict)
    :param polygons: All polygons in the graph (list)
    :return: Shapely.Polygons (list)
    """
    parks = list()

    # filter ways to parks only
    for way_id in ways:
        way = ways[way_id]
        for key in park_tags:
            if key in way.tags and way.tags[key] in park_tags[key]:
                poly = build_poly(way, nodes)
                if poly is not None:
                    parks.append(poly)
                    break  # no need to continue looping

    # filter polygons to parks only
    for polygon in polygons:
        for key in park_tags:
            if key in polygon.tags and polygon.tags[key] in park_tags[key]:
                way = extract_outerway_from_polygon(polygon, ways)
                if way is not None:
                    poly = build_poly(way, nodes)
                    if poly is not None:
                        parks.append(poly)
                        break

    return parks


def get_water(nodes, ways, polygons, waterways):
    """
    Extracts all waters from all ways, polygons and explicit waterways.
    :param nodes: All nodes in the graph (dict)
    :param ways: All ways in the graph (dict)
    :param polygons: All polygons in the graph (list)
    :param waterways: Ways with the tag 'waterway' (list)
    :return: Shapely.Polygons (list)
    """
    waters = list()

    for way_id in ways:
        way = ways[way_id]
        for key in water_tags:
            if key in way.tags and way.tags[key] in water_tags[key]:
                poly = build_poly(way, nodes)
                if poly is not None:
                    waters.append(poly)
                    break

    for polygon in polygons:
        for key in water_tags:
            if key in polygon.tags and polygon.tags[key] in water_tags[key]:
                way = extract_outerway_from_polygon(polygon, ways)
                if way is not None:
                    poly = build_poly(way, nodes)
                    if poly is not None:
                        waters.append(poly)
                        break

    for waterway in waterways:
        poly = build_poly(waterway, nodes)
        if poly is not None:
            waters.append(poly)

    return waters


def indices_from_grid(polygon, grid):
    """
    Retrieves the node indices that are close to a certain polygon, using the grid.
    :param polygon: Polygon to get neighbouring nodes for
    :param grid: Graph data as grid
    :return: indices of nodes (as are stored in the grid) as list
    """
    cell_index_nodes = []
    bounds = polygon.bounds
    if not len(bounds) == 4:
        return cell_index_nodes
    x1, y1, x2, y2 = polygon.bounds
    grid_x1, grid_y1 = grid.get_xy(Coordinate(x1, y1))
    grid_x2, grid_y2 = grid.get_xy(Coordinate(x2, y2))
    for x in range(grid_x1 - 1, grid_x2 + 2):  # -1 and +2, because we also have to check the cells just outside
        for y in range(grid_y1 - 1, grid_y2 + 2):
            if grid.inside(x, y):
                cell_index_nodes.extend(grid.data[y][x])
    return cell_index_nodes


def annotate_nodes(nodes, parks, waters):
    """
    Checks if the nodes are close to parks or waters and sets the 'water' and 'park' attributes when needed.
    :param nodes: Nodes to check (list)
    :param parks: Parks to check with (list)
    :param waters: Waters to check with (list)
    :return: Updated nodes. (list)
    """
    # build a grid for fast lookup
    minx = sys.maxint
    miny = sys.maxint
    maxx = 0
    maxy = 0
    for node in nodes:
        if node.lat < minx:
            minx = node.lat
        if node.lat > maxx:
            maxx = node.lat
        if node.lon < miny:
            miny = node.lon
        if node.lon > maxy:
            maxy = node.lon
    cell_size = (maxx - minx) / 100  # about 100x100 grid
    grid = Interval(minx, miny, maxx, maxy, None).into_grid(cell_size)

    # put the nodes in the grid
    for i in range(len(nodes)):
        node = nodes[i]
        grid.add_interval(Interval(node.lat, node.lon, node.lat, node.lon, i))

    # for all parks: test with nodes in the same cell
    print "start park compare"
    start = time.time()
    for park in parks:
        for node_index in indices_from_grid(park, grid):
            node = nodes[node_index]
            point = Point(node.lat, node.lon)
            if point.distance(park) < close_distance:
                nodes[node_index] = nodes[node_index]._replace(park=nodes[node_index].park+1)
    end = time.time()
    print "end park compare (" + str(int(end-start)) + "sec)"

    # for all waters: test nodes in the same cell
    print "start water compare"
    start = time.time()
    for water in waters:
        for index in indices_from_grid(water, grid):
            node = nodes[index]
            point = Point(node.lat, node.lon)
            if point.distance(water) < close_distance:
                nodes[index] = nodes[index]._replace(water=nodes[index].water+1)
    end = time.time()
    print "end water compare (" + str(int(end-start)) + "sec)"

    return nodes


def write_nodes_to_file(nodes):
    """
    Writes nodes (dict) to 'nodes.json' file.
    """
    nodes_dict = [node._asdict() for node in nodes]
    nodes_root = {
        "elements": nodes_dict
    }
    nodes_file = open(path_to_output_dir + "nodes.json", "w")
    nodes_file.write(json.dumps(nodes_root, indent=4))
    nodes_file.close()


def write_ways_to_file(ways):
    """
    Writes ways (dict) to 'ways.json' file.
    """
    ways_dict = [way._asdict() for way in ways.values()]
    ways_root = {
        "elements": ways_dict
    }
    ways_file = open(path_to_output_dir + "ways.json", "w")
    ways_file.write(json.dumps(ways_root, indent=4))
    ways_file.close()


def main():
    print "start reading xml"
    start = time.time()
    nodes, ways, polygons = parse_xml()
    gc.collect()  # call to garbage collector as the previous function used a few GB of RAM that is now out-of-scope
    end = time.time()
    print "end reading xml (" + str(int(end-start)) + "sec)"

    print "total nodes:    ", "{:,}".format(len(nodes))
    print "total ways:     ", "{:,}".format(len(ways))
    print "total polygons: ", "{:,}".format(len(polygons))

    highways, waterways = get_high_and_water_ways(ways)

    routable_nodes, routable_ways = get_routable_nodes_and_ways(nodes, highways)

    print "routable nodes: ", "{:,}".format(len(routable_nodes))
    print "routable ways:  ", "{:,}".format(len(routable_ways))

    write_ways_to_file(routable_ways)
    del highways
    del routable_ways
    gc.collect()

    waters = get_water(nodes, ways, polygons, waterways)
    del waterways
    gc.collect()

    print "water area's:   ", "{:,}".format(len(waters))

    parks = get_parks(nodes, ways, polygons)
    del ways
    del nodes
    del polygons
    gc.collect()

    print "park area's:    ", "{:,}".format(len(parks))

    routable_nodes = annotate_nodes(routable_nodes.values(), parks, waters)

    write_nodes_to_file(routable_nodes)


if __name__ == "__main__":
    start = time.time()
    main()
    end = time.time()
    time_in_min = (end - start) / 60
    print "TOTAL TIME (min): " + str(time_in_min)
