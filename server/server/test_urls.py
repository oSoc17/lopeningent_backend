from django.test import TestCase, Client
from django.http import HttpResponseNotFound, HttpResponseBadRequest
import json


class TestUrls(TestCase):
    """
        Very coarse grained tests.
        Mainly test whether certain requests cause certain response types.
        Occasionally also test whether certain requests yield predefined responses.
    """
    def setUp(self):
        self.client = Client()
        self.maxDiff = None

    def test_urls(self):
        """ Kind of a predefined fuzzy tester:
            All these url's are valid and should return a valid, formatted response.
        """
        randomurls = [(url + query, response)
                      for url in ['/route/generate?lat=51.0&&lon=3.8',
                                  '/route/rod?index=17263']
                      for (query, response) in [('&&type=indices', 'path'),
                                                ('&&type=coordinates', 'coordinates'),
                                                ('&&type=geojson', 'features'),
                                                ('&&type=directions', 'coordinates')]
                     ]
        for url, response in randomurls:
            url_content = self.client.get(url).content
            try:
                url_dict = json.loads(url_content)
            except BaseException:
                self.fail("Error decoding: %s from %s" % (url_content, url))
            self.assertIn(response, url_dict)


        # Querying these url's should fail, due to not-found-shit.
        url404 = ['/node?index=-5', '/node/get-id?lat=51.0&&lon=0.0',
                  '/node/get-id?lat=3.0&&lon=1.5']
        for url in url404:
            if not isinstance(self.client.get(url), HttpResponseNotFound):
                self.assertEquals(url, self.client.get(url))


        # These url's are ill-formatted and should be treated as such
        url400 = ['/node?index=party',
                  '/node/get-id?lat=5.1.0&&lon=3.6', '/node/get-id?lat=51.0']
        for url in url400:
            if not isinstance(self.client.get(url), HttpResponseBadRequest):
                self.assertEquals(url, self.client.get(url))
