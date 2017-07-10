import math
from collections import namedtuple

from server.config import EARTH_RADIUS


#
# ^ Lat
# |
# |
# |
# |       Lon
# L------->
#


class Vector(namedtuple('Vector', 'x y z')):
    """ A class containing a 3D vector."""

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    def __sub__(self, other):
        return Vector(self.x - other.x, self.y - other.y, self.z - other.z)

    def scale(self, const):
        return Vector(self.x * const, self.y * const, self.z * const)

    def dot(self, other):
        return self.x * other.x + self.y * other.y + self.z * other.z

    def cross(self, other):
        return Vector(self.y * other.z - self.z * other.y,
                      self.z * other.x - self.x * other.z,
                      self.x * other.y - self.y * other.x)

    # Returns the square length of a vector
    def lensqr(self):
        return self.dot(self)

    def unit(self):
        return self.scale(1 / math.sqrt(self.lensqr()))


class Projection(namedtuple('Projection', 'id x y')):
    """ A class representing a node projected on a plane."""

    def into_vector(self):
        return Vector(self.x, self.y, 0.0)

    @staticmethod
    def from_named(node):
        return Projection(node.id, node.x, node.y)


class Coordinate(namedtuple('Coordinate', 'x y')):
    """ A class representing a position projected on a plane."""

    def into_vector(self):
        return Vector(self.x, self.y, 0.0)

    @staticmethod
    def from_named(node):
        return Coordinate(node.x, node.y)


def get_center_node(vector_list):
    """ Returns a vector that is of unit length, and about in the middle of a vector cluster."""
    average = Vector(0, 0, 0)
    for vector in vector_list:
        average = average + vector
    average = average.scale(1. / len(vector_list))
    return average.unit()


def vector_from(lat, lon):
    """Transforms lon/lat spherical coordinates into a 3D vector.

    for example:
    (0, 0)  -> (1,0,0)
    (0, 90) -> (0,1,0)
    (90, *) -> (0,0,1)
    """
    lon_rad = lon * math.pi / 180
    lat_rad = lat * math.pi / 180
    return Vector(math.cos(lat_rad) * math.cos(lon_rad),
                  math.cos(lat_rad) * math.sin(lon_rad),
                  math.sin(lat_rad))


class Projector(object):
    """
        A class that projects nodes on a plane.

        It is meant for transforming nodes, with lat/lon coordinates, to Projections,
        with x/y coordinates, making calculations like distance and angle less accurate,
        but far easier to implement and cheaper to calculate.
    """

    def __init__(self, projection_vector, up_vector):
        self.projection_vector = projection_vector.unit()
        lenproduct = math.sqrt(projection_vector.lensqr() * up_vector.lensqr())
        scale_factor = up_vector.dot(projection_vector) / lenproduct
        self.up_vector = (
            up_vector - projection_vector.scale(scale_factor)).unit()
        self.remaining = self.projection_vector.cross(self.up_vector).unit()

    def map(self, node):
        point = vector_from(node.lat, node.lon)
        y = self.up_vector.dot(point) * EARTH_RADIUS
        x = self.remaining.dot(point) * EARTH_RADIUS
        return Coordinate(x, y)


def project_city(node_list):
    """ Returns a list of Projections mirroring a list of Nodes which represent a city."""
    x = map(lambda node: vector_from(node.lat, node.lon), node_list)
    average = get_center_node(x)
    projector = Projector(average, Vector(0, 0, 1))
    return projector
