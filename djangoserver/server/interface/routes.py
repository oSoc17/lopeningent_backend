from server.static.city import GRAPH
from django.http import HttpResponse, HttpResponseNotFound
from server.logic.server_util import into_json
from server.logic.routing import config as routing_config
from server.logic.routing.routing import generate_rod, close_rod
from server.logic.routing.compress import from_string
from server.config import DEFAULT_ROUTING_CONFIG
from server.interface.geojson import respond_path
from server.logic.city.city import path_length
from server.interface.util import get_edge_tuple
from server.logic.graph.util import distance
from server.interface.util import serialize_node
from random import shuffle
from django.views.decorators.csrf import csrf_exempt
import json

from server.logic.routing.compress import into_string
from server.logic.routing.ratings import add_rating_list

from server.setdebug import NUM_THREADS

from concurrent.futures import ThreadPoolExecutor as Executor, wait, FIRST_COMPLETED


def async_exec(graph, start, end, config):
    # Create the rod.
    nodes = generate_rod(graph, start, config)

    # Fix that the rod doesn't pass our position
    if nodes[1] == end:
        nodes = nodes[1:]
        (start, end) = (end, start)

    # Close the rod.
    return close_rod(graph, end, nodes, config)


def route_from_coord(request):
    """
        Responds with a route starting and ending in a certain coordinate

        This function uses the lightning rod (Stroobant) algorithm: it generates a
        rod through the city and then tries different paths going to that rod

        Query args:
        lat -- coordinate latitude
        lon -- coordinate longitude
        type -- the response type. See the geojson.respond_path function for more details
    """
    edge_tuple = get_edge_tuple(request, request.GET.get('lat'), request.GET.get('lon'))
    if edge_tuple is None:
        # This happens when the requested coordinates are not within the Graph.
        return HttpResponseNotFound()
    (start, end) = edge_tuple
    config = routing_config.from_dict(
        DEFAULT_ROUTING_CONFIG, request.GET.dict())

    # Calculations
    tpexec = Executor(max_workers=NUM_THREADS)
    routes = []
    flist = []

    # Sometimes, routing gives a bad response. To fix this, the routing algorithm
    # is performed multiple times in an (a)synchronous way.
    for i in xrange(NUM_THREADS):
        flist.append(tpexec.submit(async_exec, GRAPH, start, end, config))

    for _i in xrange(NUM_THREADS * 10):
        sets = wait(flist, return_when=FIRST_COMPLETED)
        flist = list(sets.not_done)
        for fut in sets.done:
            routes = fut.result()
            if len(routes) > 0:
                break
            flist.append(tpexec.submit(async_exec, GRAPH, start, end, config))

    tpexec.shutdown(False)

    # Choose a random route from all possible routes.
    shuffle(routes)
    print path_length(GRAPH, routes[0])
    resp = respond_path(request.GET, routes[0], [routes[0][0]])
    if resp is None:
        return HttpResponseNotFound()
    return HttpResponse(into_json(resp))


def rod(request):
    """
        Responds with a rod starting from an index

        This function is mostly useless and should - currently - only be used for
        testing and/or speed metrics.
    """
    index = int(request.GET.get('index'))
    config = routing_config.from_dict(
        DEFAULT_ROUTING_CONFIG, request.GET.dict())
    nodes = generate_rod(GRAPH, index, config)
    resp = respond_path(request.GET, nodes, [])
    if resp is None:
        return HttpResponseNotFound()
    return HttpResponse(into_json(resp))


def parse(request):
    """
        Convert a tag into another type.

        Query args:
        tag -- route to be converted.

    """
    tag = request.GET.get('tag')
    path = from_string(GRAPH, tag)
    resp = respond_path(request.GET, path, [path[0]])
    if resp is None:
        return HttpResponseNotFound()
    return HttpResponse(into_json(resp))


def go_home(request):
    """
        Responds with a route leading the user back to his starting point.

        Query args:
        tag -- the route that the user used to run.
        lon, lat -- position of the user.
        distance -- The preferred distance to the starting point.
    """
    # Get path from request tag
    tag = request.GET.get('tag')
    path = from_string(GRAPH, tag)

    # Find nearest node
    (start, end) = get_edge_tuple(request, request.GET.get('lat'), request.GET.get('lon'))

    # Get the preferred distance to run from this point to starting position (0 means straight home)
    dist_arg = float(request.GET.get('distance'))
    if dist_arg == 0:
        dist = distance(serialize_node(GRAPH, end), serialize_node(GRAPH, path[0]))
    else:
        dist = dist_arg
    print dist

    # Get index of current location and close rod to return
    if end not in path and start in path:
        (end, start) = (start, end)
    elif not (end in path or start in path):
        for node in serialize_node(GRAPH, end).connections + serialize_node(GRAPH, start).connections:
            if node.node in path:
                end = node.node
                break
    ind = path.index(end)

    # Nodes from starting to current position
    pois_path = path[ind:0:-1]
    dist = dist - path_length(GRAPH, pois_path)

    d = {k: v for k, v in request.GET.dict().items()}
    d['min_length'] = str(dist)
    d['max_length'] = str(dist + 1.0)
    config = routing_config.from_dict(DEFAULT_ROUTING_CONFIG, d)

    # Generate new random rod from starting position
    nodes = generate_rod(GRAPH, path[0], config)
    # Create new rod that will be used for the poisoning (starting from current position and contains starting pos)
    pois_path.extend(nodes)
    print "tag =", into_string(GRAPH, nodes)
    # Close the rod on the starting position
    routes = close_rod(GRAPH, end, pois_path, config, nodes)
    # Will result in the shortest route returned
    if dist_arg == 0:
        routes = sorted(routes, key=len)
    else:
        shuffle(routes)
    print path_length(GRAPH, routes[0])
    # Return the new route, i.e. the completed part + new part
    selected_route = path[0:ind] + routes[0][::-1]
    resp = respond_path(request.GET, selected_route, [selected_route[0]])
    if resp is None:
        return HttpResponseNotFound()
    return HttpResponse(into_json(resp))


@csrf_exempt
def convert(request):
    """
        Converts a route into another type

        Deprecated and replaced by 'parse'
    """
    if len(request.body) > 0:
        body = json.loads(request.body)
    else:
        body = {}
    body.update({key: request.GET.get(key) for key in request.GET})
    print(body)
    resp = respond_path(body, body['data'], [body['data'][0]])
    if resp is None:
        return HttpResponseNotFound()
    return HttpResponse(into_json(resp))


def rate(request):
    """
        Adds the rating to all edges in a route, and saves it both in the structure and in the database.

        Query args:
        tag -- the tag of the route you want to rate
        rating -- a float between 0 and 5
    """

    tag = request.GET.get('tag')
    rating = float(request.GET.get('rating'))
    path = from_string(GRAPH, tag)
    add_rating_list(GRAPH, [(i, j) for i, j in zip(path, path[1:])], rating)
    return HttpResponse('')


# route/import?file=trimpistes_0.4.json
# route/import?file=running_routes_tags_0.4.json

@csrf_exempt
def import_json(request):
    """
        Processes a json structure that contains route and applies the score in the MongoDB database.
        The structure should have the following structure:
        {
            'route_id (e.g. 1)' : { 'length': int, 'score': int, 'name': string, 'tags': arrays[string] }
        }

        The data segment should contain the structure
            (can be done through e.g. curl "<domain>/route/import" --data "@filename")
    """
    print request.body
    jsonobj = json.loads(request.body)

    for key in jsonobj:
        tags = jsonobj[key]['tags']
        score = float(jsonobj[key]['score'])
        for tag in tags:
            path = from_string(GRAPH, tag)
            add_rating_list(GRAPH, [(i, j) for i, j in zip(path, path[1:])], score)
    return HttpResponse('<html><body>Imported ' + file_name + '</body></html>')
