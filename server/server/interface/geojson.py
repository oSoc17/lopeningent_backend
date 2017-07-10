from collections import namedtuple
from server.static.city import GRAPH
from server.logic.routing.directions import into_directions, DIRECTION_DICTIONARY
from server.logic.city.city import path_length
from server.logic.routing.compress import into_string


class GeoCoord(namedtuple("GEOCOORD", "lat lon")):
    @staticmethod
    def from_named(named):
        return GeoCoord(named.lat, named.lon)


def respond_path(request_dict, path, markers):
    """ Wraps a path into a response object, given the request as sent to the server.

        Keyword arguments:
        request -- The request as received by the server
        path -- a set of graph indices.
        markers -- a set of graph indices.

        Query arguments:
        type -- the response type (default: "indices"). Possible values are:
            "indices": return just the list of indices;
            "coordinates": return a list of coordinates which are interpretable by the app;
            "geojson": return a geojson object.
            "length" : return length information, not the route.
            "debug" : useful information for debugging.
    """
    tag_path = path
    response_type = request_dict.get("type", "indices")
    resp = None
    # Indices
    if response_type == "indices":
        class ResponseTupleIndices(namedtuple('RT', "path")):
            pass
        resp = ResponseTupleIndices(path)

    # Directions
    elif response_type == "directions":
        path = into_directions(GRAPH, path, DIRECTION_DICTIONARY)

        class ResponseTupleDirections(namedtuple('RT', "coordinates")):
            pass
        resp = ResponseTupleDirections(path)

    # Retrieve the distance. Useful for tests
    elif response_type == "length":
        class ResponseTupleDist(namedtuple('RT', 'length perceived')):
            pass
        distance_fn = lambda x: x.distance * x.modifiers.highway
        perceived_length = path_length(GRAPH, path, distance_fn)
        length = path_length(GRAPH, path)
        resp = ResponseTupleDist(length, perceived_length)

    # debugging
    elif response_type == "debug":
        path = into_directions(GRAPH, path, DIRECTION_DICTIONARY)
        markers = [coord for coord in path if coord.c ==
                   request_dict.get("filter", "forward")]
    else:
        path = [GeoCoord.from_named(GRAPH.get(ident)) for ident in path]
        markers = [GeoCoord.from_named(GRAPH.get(ident)) for ident in markers]

    # Lat-Lon coordinates
    if response_type == "coordinates":
        class ResponseTupleLatLon(namedtuple('RT', "coordinates markers")):
            pass
        resp = ResponseTupleLatLon(path, markers)

    # GeoJSON format
    elif response_type == "geojson" or response_type == "debug":
        class ResponseTupleGeo(namedtuple('RT', "type features")):
            pass

        class Feature(namedtuple('F', 'type geometry')):
            pass

        class Geometry(namedtuple('G', 'type coordinates')):
            pass
        points = [Feature("Feature", Geometry("Point", [coord.lon, coord.lat])) for coord in markers]
        lines = [Feature("Feature", Geometry("LineString", [[coord.lon, coord.lat] for coord in path]))]
        resp = ResponseTupleGeo("FeatureCollection", points + lines)

    # No format found
    res_dict = resp._asdict()
    if response_type != "geojson":
        res_dict['tag'] = into_string(GRAPH, tag_path)
    return res_dict
