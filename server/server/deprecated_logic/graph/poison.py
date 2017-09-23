from server.logic.graph.graph import lib, ffi, DijkstraIterator


class PoisonedGraph(object):
    """
        A class that represents a poisoned graph
        This class basically overwrites and saves a subset of the original graph,
        and refers to the original graph itself if the data inside is not found.
    """
    def __init__(self, origin, rod, max_distance, max_value):
        rod_c = ffi.new("size_t[]", rod)
        self.poison = lib.graph_poison(origin.graph, rod_c, len(rod), max_distance, max_value)

    def __del__(self):
        lib.graph_poison_delete(self.poison)

    def generate_dijkstra(self, start_node, config):
        class DijkstraIteratorPoison(DijkstraIterator):
            def __init__(self, graph, start_node, config):
                self.graph = graph
                configuration = self.gen_config(config)
                self.dijkstra = lib.graph_poison_generate(graph, start_node, configuration)

            def __del__(self):
                lib.graph_dijkstra_delete(self.dijkstra)

        return DijkstraIteratorPoison(self.poison, start_node, config)
