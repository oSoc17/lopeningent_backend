"""
    Various distance functions.
"""


from server.logic.projection.util import Projection, Coordinate

from collections import namedtuple


def distance_to_edge_sqr(point, edge_start, edge_end):
    """ Retrieves the distance between a segment (edge) and a point.

        Function args:
        point -- a Vector of the point
        edge_start -- a Vector of the start point of the edge
        edge_end -- a Vector of the endpoint of the edge
        Returns -- the distance, measured in km
    """
    if (point - edge_end).dot(edge_start - edge_end) < 0:
        return (point - edge_end).dot(point - edge_end)
    elif (point - edge_start).dot(edge_end - edge_start) < 0:
        return (point - edge_start).dot(point - edge_start)
    else:
        return (point - edge_start).cross(edge_end - edge_start).lensqr()


def get_closest_edge(coord, graph, grid):
    """
        Given a coordinate, a list of nodes and a list of edges, return the edge closest to a point.

        Function args:
        coord -- object with xy attributes
        graph -- owning graph
        grid -- grid generated from the graph

        Returns two nodes, connected according to their owning Graph.
    """
    curr_dist = None
    curr_tuple = None
    for interval in grid.get(coord):
        dist = distance_to_edge_sqr(
            Coordinate.from_named(coord).into_vector(),
            Projection.from_named(graph.get(interval[0])).into_vector(),
            Projection.from_named(graph.get(interval[1])).into_vector()
        )
        if curr_dist is None or dist < curr_dist:
            curr_dist = dist
            curr_tuple = interval
    return curr_tuple


def dot_distance(start_a, end_a, start_b, end_b):
    return (end_a.x - start_a.x) * (end_b.x - start_b.x) + (end_a.y - start_a.y) * (end_b.y - start_b.y)


def bird_distance(point_a, point_b):
    """ Simply return the length of a straight line between the two points. """
    return dot_distance(point_a, point_b, point_a, point_b)**0.5


def angle(point_a, mid, point_c):
    class Point(namedtuple('XY', 'x y')):
        pass
    mid2 = Point(mid.y, -mid.x)
    c2 = Point(point_c.y, -point_c.x)
    return dot_distance(point_a, mid, mid2, c2) / bird_distance(point_a, mid) / bird_distance(mid, point_c)
