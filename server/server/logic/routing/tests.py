from django.test import TestCase
from server.logic.routing.util import ground, unground
from server.static.city import GRAPH
from server.logic.routing.poison import poison_graph
from server.logic.routing.config import from_dict
from server.config import DEFAULT_ROUTING_CONFIG
from server.logic.routing.compress import into_string, from_string
from server.logic.routing.routing import generate_rod


class TestUtil(TestCase):
    """
        Test whether the ground and unground functions work
    """

    def test_ground_unground(self):
        l = [1, 2, 3, 4, 5, 6]
        self.assertEqual(l, ground(unground(l), l[len(l) - 1]))

class TestPoison(TestCase):
    """
        Test whether poisoning doesn't change the graph
    """
    """
        Due to the limitations of the poison class, this test has lost its use
    """
    def poison(self):
        graph = GRAPH
        config = from_dict(DEFAULT_ROUTING_CONFIG, {'poison_max_distance' : "10.0", 'poison_max_value' : "50.0"})
        poison = poison_graph(graph, [17263], config)
        self.assertEqual(len([i for i in graph.list_ids()]), len([i for i in poison.list_ids()]))
        self.assertEqual(sorted([i for i in graph.list_ids()]), sorted([i for i in poison.list_ids()]))
        for node_id in poison.list_ids():
            self.assertEqual(set(graph.get_connids(node_id)), set(poison.get_connids(node_id)))

class TestEncodeDecode(TestCase):
    """
        Test whether encoding or decoding actually works
    """
    def test_encode_decode(self):
        rod = generate_rod(GRAPH, 17263, from_dict(DEFAULT_ROUTING_CONFIG, {}))
        string = into_string(GRAPH, rod)
        self.assertEqual(from_string(GRAPH, string), rod)
