"""
    This file implements the Lightning Rod algorithm for route generation

    The algorithm consists of three steps:
     1. A "rod" is generated, a shortest path between the starting node and a random
        node at a set distance.
     2. Various routes are generated from the starting node towards the rod. When
        they collide, the path formed by the lower part of the rod and the generated
        path is tested on length. If the length satisfies the total length constraint,
        it is added to the list of possible results.
     3. The best path, using other cost metrics, is selected.

"""

from server.logic.routing.util import RandomChooser, ground, unground
from server.logic.routing.poison import poison_graph


def generate_rod(graph, start_node, routing_config):
    """
        Generates a random rod.
    """

    dijkstra = graph.generate_dijkstra(start_node, routing_config).choose(routing_config)
    (node_id, _) = dijkstra.next()
    res = dijkstra.root(node_id)
    return res[::-1]


def close_rod(graph, start_node, rod, routing_config, alt_rod=None):
    """
        Generates a closed route given a rod
    """
    if alt_rod is None:
        alt_rod = rod
    res = []

    # Poison
    poisoned_graph = poison_graph(graph, rod, routing_config)

    # Map the distance from the starting node to the nodes in the rod.
    distance_dict = annotate_rod(
        alt_rod, graph, lambda x: x.distance)

    # Contains alt_rod[y] -> y
    rod_pos = {node_id: n for n, node_id in enumerate(alt_rod)}

    dijkstra = poisoned_graph.generate_dijkstra(start_node, routing_config).filter(alt_rod)
    for node_id, (size, actual) in dijkstra:
        total_length = actual + distance_dict[node_id]
        res.append((node_id, total_length))

    print "Routes found:", len(res)

    # Remove routes that are not of satisfying length
    filtered_res = filter(lambda (_, tl): tl < routing_config.max_length and tl > routing_config.min_length, res)

    # Calculate the actual route.
    filtered_res_rooted = [(alt_rod[:rod_pos[node_id] + 1] +
                            dijkstra.root(node_id), length) for (node_id, length) in filtered_res]
    print "Routes selected:", len(filtered_res_rooted)
    return [route for route, _ in filtered_res_rooted]


def annotate_rod(rod, graph, distance_fn):
    """
        Returns a dictionary with information about distances from the rod to the starting node.
    """
    length = 0.0
    prev_dict = {}
    prev_dict[rod[0]] = 0.0
    for (prev_node, next_node) in zip(rod, rod[1:]):
        length += filter(lambda x: x.to == next_node, graph.get_edges(
            prev_node))[0].distance
        prev_dict[next_node] = length
    return prev_dict
