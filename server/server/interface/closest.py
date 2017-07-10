from django.http import HttpResponse, HttpResponseNotFound, HttpResponseBadRequest
from server.logic.server_util import into_json
import server.static.city as city
from server.interface.util import get_edge_tuple

# /node/get-id?lat=<lat>&&lon=<lon>


def get_id_from_pos(request):
    """
        Respond with the node index closest to the given coordinate.

        Query args:
        lat -- coordinate latitude
        lon -- coordinate longitude
    """
    try:
        tup = get_edge_tuple(request, request.GET.get(
            'lat'), request.GET.get('lon'))
        if tup is None:
            return HttpResponseNotFound()
        return HttpResponse(into_json(tup))
    except BaseException:
        return HttpResponseBadRequest()

# /node/get-node?lat=<lat>&&lon=<lon>


def get_node_from_pos(request):
    """
        Respond with the node data closest to the given coordinate.

        Query args:
        lat -- coordinate latitude
        lon -- coordinate longitude
    """
    try:
        tup = get_edge_tuple(request, request.GET.get(
            'lat'), request.GET.get('lon'))
    except BaseException:
        return HttpResponseBadRequest()
    if tup is None:
        return HttpResponse('[]')
    tup = map(city.GRAPH.get, tup)
    return HttpResponse(into_json(tup))
