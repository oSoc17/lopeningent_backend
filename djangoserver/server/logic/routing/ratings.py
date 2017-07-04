from server.logic.city.city import DATABASE
from server.logic.graph.graph import lib


def add_rating_list(graph, edges, rating):
    """
    Adds a rating to an edge in the database. If an edge does not exist, it is created.

    Function args:
    graph -- the graph
    edges -- the edges which will get a rating
    rating -- the score which is passed
    """
    DATABASE.add_rating_list([(start_node, end_node, rating) for start_node, end_node in edges])
    for start_node, end_node in edges:
        lib.graph_update_rating(graph.graph, start_node, end_node, rating)
