from django.test import TestCase
from collections import namedtuple
from server.static.city import GRAPH
from server.static.city import GRID

class GraphTestCase(TestCase):
    """
        Test the size of the GRID
    """
    def test(self):
        self.assertEquals((GRID.height, GRID.width), (598, 887))
