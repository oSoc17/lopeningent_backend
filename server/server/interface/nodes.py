from django.http import HttpResponse, HttpResponseNotFound, HttpResponseBadRequest
from server.logic.server_util import into_json
import server.static.city as city
from server.logic.city.city import Vertex

from server.interface.util import serialize_node

# /node?index=<index>


def get_node(request):
    """ Responds with information about a node.

        Query args:
        index -- the index of the node
    """
    try:
        index = int(request.GET.get('index'))
    except BaseException:
        return HttpResponseBadRequest()
    if city.GRAPH.contains(index):
        return HttpResponse(into_json(serialize_node(city.GRAPH, index)))
    else:
        return HttpResponseNotFound()


def in_city(request):
    """ Responds whether a coordinate is inside the city or not

        Query args:
        lat -- coordinate latitude
        lon -- coordinate longitude
    """
    try:
        lat = float(request.GET.get('lat'))
        lon = float(request.GET.get('lon'))
    except BaseException:
        return HttpResponseBadRequest()
    coord = city.PROJECTOR.map(Vertex(0, 0, lat, lon, 0, 0, []))
    (x, y) = city.GRID.get_xy(coord)
    if city.GRID.inside(x, y):
        return HttpResponse("true")
    else:
        return HttpResponse("false")
