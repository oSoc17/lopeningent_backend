from server.logic.graph.poison import PoisonedGraph


def poison_graph(graph, venomous_path, config):
    """
        Poisons a graph

        Function args:
        graph -- the graph
        venomous_path -- the path from which the poison originates
        config -- routing configuration
    """
    return PoisonedGraph(graph, venomous_path, config.poison_max_distance, config.poison_max_value)
