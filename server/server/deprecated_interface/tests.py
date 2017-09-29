from django.test import TestCase
import server.interface.nodes as views
from django.http import HttpResponse, HttpResponseNotFound, HttpResponseBadRequest
import server.interface.closest as cviews
import server.interface.util as util
from django.test.client import RequestFactory

class NodeTestCase(TestCase):
    """
        Test the node interface.
        Kind of obsolete due to test_urls, but still useful
    """

    def setUp(self):
        self.rf = RequestFactory()

    def test_get_edge_tuple(self):
        util.get_edge_tuple(None, 51.0, 3.8)

    def test_get_node(self):
        """
            Fuzzy test
            Test whether indexing yields the correct response type
        """
        node = views.get_node(self.rf.get('/node?index=0'))
        self.assertTrue(isinstance(node, HttpResponse))
        self.assertFalse(isinstance(node, HttpResponseNotFound))
        node = views.get_node(self.rf.get('/node?index=-5'))
        self.assertTrue(isinstance(node, HttpResponseNotFound))

    def test_get_from(self):
        """
            Fuzzy tests.
            They test whether the function returns the correct response kind.
        """
        node = cviews.get_id_from_pos(
            self.rf.get('/node?lat=51.0&lon=3.8'))
        self.assertTrue(isinstance(node, HttpResponse))
        node = cviews.get_node_from_pos(
            self.rf.get('/node?lat=5.1.0&lon=3.8'))
        self.assertTrue(isinstance(node, HttpResponseBadRequest))
        node = cviews.get_id_from_pos(
            self.rf.get('/node?lat=51.0&lon=3.8.0'))
        self.assertTrue(isinstance(node, HttpResponseBadRequest))

    def test_in_city(self):
        """
            Test whether the coordinates are inside/outside the city
        """
        self.assertEquals(views.in_city(self.rf.get('/node?lat=51.0&lon=3.8')).content, "true")
        self.assertEquals(views.in_city(self.rf.get('/node?lat=51.0&lon=2.8')).content, "false")
        self.assertTrue(isinstance(views.in_city(self.rf.get('/node?lat=51.0&lon=3.8.0')), HttpResponseBadRequest))
