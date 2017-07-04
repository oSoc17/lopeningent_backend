from server.logic.grid.interval import Interval
from django.test import TestCase
from collections import namedtuple
from server.logic.projection.util import Coordinate


class TestNode(namedtuple('Node', 'id')):
    pass


class GridTestCase(TestCase):
    """
        Example of how to use a grid.
    """

    def setUp(self):
        self.grid = Interval(0.0, 0.0, 10.0, 5.0, None).into_grid(3.0)

    def test_grid_creation(self):
        """
            Test grid creation
        """
        self.assertEqual(self.grid.width, 4)
        self.assertEqual(self.grid.height, 2)
        self.assertEqual(self.grid.get_xy(Coordinate(1.0, 1.0)), (0, 0))
        self.assertEqual(self.grid.get_xy(Coordinate(9.0, 4.0)), (3, 1))

    def test_grid_addition(self):
        """
            Test grid addition
        """
        self.grid.add_interval(Interval(0.0, 0.0, 4.0, 5.0, 5))
        self.grid.add_interval(Interval(3.0, 3.0, 6.0, 4.0, 7))
        self.assertEqual(self.grid.get(Coordinate(3.0, 2.0)), [5])
        self.assertEqual(self.grid.get(Coordinate(2.5, 4.0)), [5, 7])
        self.assertEqual(self.grid.get(Coordinate(6.0, 4.0)), [7])
        self.assertEqual(self.grid.get(Coordinate(9.0, 2.0)), [])
