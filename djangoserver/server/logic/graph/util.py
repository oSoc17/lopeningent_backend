import math

from server.config import EARTH_RADIUS


def haversine(rad):
    """
        Returns the haversine function of an angle in radians.
    """
    return (1 - math.cos(rad)) / 2


def distance(startnode, endnode):
    """
        Returns the distance between two nodes in km.
    """
    phi1 = startnode.lat * math.pi / 180
    phi2 = endnode.lat * math.pi / 180
    dphi = phi1 - phi2
    dtheta = (startnode.lon - endnode.lon) * math.pi / 180
    return 2 * EARTH_RADIUS * math.asin(
        math.sqrt(haversine(dphi) + math.cos(phi1) *
                  math.cos(phi2) * haversine(dtheta))
    )

def parse(data):
    res = "\n".join(string for string in data.splitlines()
                    if not (string.startswith("#") or string == "}" or string.startswith("extern")))
    return res
