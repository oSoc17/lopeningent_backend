from django.test import TestCase
from server.static.city import GRAPH, GRID
from server.logic.distance.util import get_closest_edge
from server.logic.projection.util import Projection

TEST_COUNT = 100


class ClosestTestCase(TestCase):
    """
        Class for testing whether the get_closest_edge functions work
    """
    def test_closest_edge_single_point(self):
        """
            This test checks that every node in the graph is closest to an edge that
            contains the node.
        """
        i = 0
        for ident in GRAPH.list_ids():
            if len(GRAPH.get_connids(ident)) == 0:
                continue
            i += 1
            if i == TEST_COUNT:
                return
            point = GRAPH.get(ident)
            closest_tuple = get_closest_edge(point, GRAPH, GRID)
            if not (ident == closest_tuple[0] or ident == closest_tuple[1]):
                print(ident, closest_tuple)
                self.assertTrue(False)


    def test_closest_edge_multi_point(self):
        """
            This test checks whether the api will localize all street midpoints on the
            correct egde, so that the closest edge to the point is the actual edge that
            created the point.
        """
        i = 0
        for primary in GRAPH.list_ids():
            for secondary in GRAPH.get_connids(primary):
                i += 1
                if i == TEST_COUNT:
                    return
                point = GRAPH.get(primary)
                point2 = GRAPH.get(secondary)
                newpoint = Projection(-1, (point.x + point2.x) /
                                      2, (point.y + point2.y) / 2)
                closest_tuple = get_closest_edge(newpoint, GRAPH, GRID)
                self.assertEqual(sorted((point.id, point2.id)),
                                 sorted(closest_tuple))
