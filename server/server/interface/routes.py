from server.static.data import GRAPH, DATABASE_EDGES
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
from server.database import update_edge_in_db
from server.setdebug import NUM_THREADS
from concurrent.futures import ThreadPoolExecutor as Executor, wait, FIRST_COMPLETED
import json, logging


@csrf_exempt
def generate(request):
    """
    Responds with a route starting and ending in a certain coordinate

    Query args:
    lat -- coordinate latitude
    lon -- coordinate longitude
    tags -- the requested POI's
    type -- the response type. See the geojson.respond_path function for more details
    """
    try:
        logging.info("static route generator: incoming request")
        badtags = request.POST.getlist('neg_tags')
        usertags = request.POST.getlist('tags')
        lat = float(request.POST.get('lat'))
        lon = float(request.POST.get('lon'))
        edge_tuple = get_edge_tuple(request, lat, lon)

        if edge_tuple is None:
            # This happens when the requested coordinates are not within the Graph.
            return HttpResponseNotFound("Can't find a starting point near to your location.")
        (start, end) = edge_tuple
        config = routing_config.from_dict(
            DEFAULT_ROUTING_CONFIG, request.POST.dict())

        logging.debug("""static route generator: parameters:
            lat/lon(%s, %s),
            ptags(%s),
            ntags(%s)
            minlen(%s),
            maxlen(%s),
            type(%s")""",
            lat, lon, usertags, badtags, config.min_length, config.max_length, request.POST.get('type'))

        def calculate_modifier(edge):
            dbe = DATABASE_EDGES[edge.id]
            edge.modifier += (dbe.rating / 10)

            for tag in dbe.tags:
                if tag in usertags:
                    edge.modifier += (1 / (len(usertags) * 2))
                elif tag in badtags:
                    new_mod = (1 / (len(badtags) * 2))

                    if new_mod < 0:
                        new_mod = 0

                    edge.modifier -= new_mod

            return edge

        def async_exec(graph, start, end, config):
            # Create the rod.
            nodes = generate_rod(graph, start, config)

            # Fix that the rod doesn't pass our position
            if nodes[1] == end:
                nodes = nodes[1:]
                (start, end) = (end, start)

            # Close the rod.
            return close_rod(graph, end, nodes, config)

        # Calculations
        tpexec = Executor(max_workers=NUM_THREADS)
        routes = []
        flist = []

        logging.info("static route generator: mapping modifiers to edges (rating + tags)")
        GRAPH.map_graph(lambda _: _, calculate_modifier)

        # Sometimes, routing gives a bad response. To fix this, the routing algorithm
        # is performed multiple times in an (a)synchronous way.
        logging.info("static route generator: executing the routing algorithm")
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
        logging.info("static route generator: choosing a random route out of all posibilities")
        shuffle(routes)
        resp = respond_path(request.POST, routes[0], [routes[0][0]])
        if resp is None:
            logging.error('static route generator: no route available', exc_info=True)
            return HttpResponseNotFound("No routes found for this parameter set.")
        logging.debug("static route generator: generated route: %s", into_json(resp))
        logging.info("static route generator: sending a response with a route")
        return HttpResponse(into_json(resp))
    except:
        logging.error('static route generator: failed to generate a static route', exc_info=True)
        return HttpResponseNotFound("Something went wrong with your request.")


@csrf_exempt
def return_home(request):
    """
    Responds with a route leading the user back to his starting point.

    Query args:
    visited_path -- the route that the user used to run.
    lon, lat -- position of the user.
    distance -- The preferred distance to the starting point.
    """
    try:
        logging.info("dynamic route generator: incoming request")
        # Get path from request tag
        tag = request.POST.get('visited_path')
        path = from_string(GRAPH, tag)
        lat = float(request.POST.get('lat'))
        lon = float(request.POST.get('lon'))

        # Find nearest node
        (start, end) = get_edge_tuple(request, lat, lon)

        # Get the preferred distance to run from this point to starting position (0 means straight home)
        dist_arg = float(request.POST.get('distance'))
        if dist_arg == 0:
            dist = distance(serialize_node(GRAPH, end), serialize_node(GRAPH, path[0]))
        else:
            dist = dist_arg

        logging.debug("""dynamic route generator: parameters:
            tag (%s),
            path (%s),
            lat/lon(%s, %s),
            distance(%s),
            """,
            tag, path, lat, lon, distance)

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

        d = {k: v for k, v in request.POST.dict().items()}
        d['min_length'] = str(dist)
        d['max_length'] = str(dist + 1.0)
        config = routing_config.from_dict(DEFAULT_ROUTING_CONFIG, d)

        # Generate new random rod from starting position
        nodes = generate(GRAPH, path[0], config)
        # Create new rod that will be used for the poisoning (starting from current position and contains starting pos)
        pois_path.extend(nodes)
        # Close the rod on the starting position
        routes = close_rod(GRAPH, end, pois_path, config, nodes)
        # Will result in the shortest route returned
        if dist_arg == 0:
            routes = sorted(routes, key=len)
        else:
            shuffle(routes)
        # Return the new route, i.e. the completed part + new part
        selected_route = path[0:ind] + routes[0][::-1]
        resp = respond_path(request.POST, selected_route, [selected_route[0]])
        if resp is None:
            logging.error('dynamic route generator: no route available', exc_info=True)
            return HttpResponseNotFound("No routes found for this parameter set.")
        logging.debug("dynamic route generator: generated route: %s", into_json(resp))
        logging.info("dynamic route generator: sending a response with a route")
        return HttpResponse(into_json(resp))
    except:
        logging.error('dynamic route generator: failed to generate a static route', exc_info=True)
        return HttpResponseNotFound("Something went wrong with your request.")


@csrf_exempt
def rate_route(request):
    """
    Adds the rating to all edges in a route, and saves it both in the structure and in the database.

    Query args:
    visited_path -- the route's tag
    rating -- a float between 0 and 5
    """
    try:
        logging.info("RATING: received rating call")
        tag = request.POST.get('visited_path')
        new_rating = float(request.POST.get('rating'))
        path = from_string(GRAPH, tag)
        edgeids = [(f, t) for f, t in zip(path, path[1:])]

        def update_rating(edge):
            for from_id, to_id in edgeids:
                if edge.id == from_id and edge.to == to_id:
                    update_edge_in_db(edge, new_rating)

            return edge

        logging.info("RATING: mapping ratings to the graph edges")
        GRAPH.map_graph(lambda _:_, update_rating)
        return HttpResponse('')
    except:
        logging.error("RATE: we've run into an error while rating a route", exc_info=True)
        return HttpResponseNotFound("Something went wrong while rating your route.")
