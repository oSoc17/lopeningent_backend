from django.test import TestCase
from server.logic.graph.graph import Graph
from server.logic.graph.poison import PoisonedGraph


class TestExample(TestCase):
    """
        Example of how to use this graph library
        Please read this carefully if you're going to work with it.
    """
    """
        While this example is still useful in understanding how the graph works,
        the example itself does not work anymore, since the ffi boundary screwed
        dynamic graph entries.
        Check the Rust tests for more information.
    """

    def example(self):
        nodes = [(0, "A"), (1, "B"), (2, "C"), (3, "D")]
        edges = [(0, "AB", 1), (1, "BC", 2), (2, "CA", 0), (0, "AD", 3)]
        graph = Graph(nodes, edges)

        self.assertEquals(graph.get(0), "A")
        self.assertEquals(graph.get(1), "B")

        self.assertEquals(graph.get_conn_idval(0), [(1, "AB"), (3, "AD")])
        self.assertEquals(graph.get_edges(0), ["AB", "AD"])
        self.assertEquals(graph.get_connids(0), [1, 3])

        self.assertEquals(sorted(graph.list_ids()), [0, 1, 2, 3])

        generator = graph.gen_dijkstra(0, lambda _: 1, lambda x: -len(x))
        collect = [(graph.get(node_id), length)
                   for node_id, length, _ in generator]

        self.assertEquals(collect,
                          [("A", 0), ("B", -2), ("D", -2), ("C", -4)])

        graph = graph.map_graph(lambda n: n * 2, lambda e: e * 3)

        self.assertEquals(graph.get(0), "AA")
        self.assertEquals(graph.get_edges(0), ["ABABAB", "ADADAD"])

        nodes = [(0, "A")]
        edges = [(0, "AD", 3)]

        poison = PoisonedGraph(nodes, edges, graph)

        self.assertEquals(poison.get(0), "A")
        self.assertEquals(poison.get(1), "BB")

        generator = poison.gen_dijkstra(0, lambda _: 1, lambda x: -len(x))
        collect = [(poison.get(node_id), length)
                   for node_id, length, _ in generator]

        self.assertEquals(collect,
                          [("A", 0), ("BB", -6), ("DD", -2), ("CC", -12)])

        generator = poison.gen_dijkstra(0, lambda x: len(x), lambda x: -len(x))
        collect = [(poison.get(node_id), length)
                   for node_id, length, _ in generator]

        self.assertEquals(collect,
                          [("A", 0), ("DD", -2), ("BB", -6), ("CC", -12)])
