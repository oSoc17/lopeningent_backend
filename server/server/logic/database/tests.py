import unittest
from server.logic.database.edge_database import EdgeDatabase
from pymongo import MongoClient
from server.config import DATABASE_CONNECTION
from server.config import DATABASE_SCHEME_EDGE_RATING


class DatabaseTestCase(unittest.TestCase):
    def setUp(self):
        """
        Set up database class and instruct to use test database.
        """
        self.database = EdgeDatabase()
        self.database.__database_name__ = "servertest"

    def tearDown(self):
        """
        Wipe test database.
        """
        client = MongoClient(DATABASE_CONNECTION['host'], DATABASE_CONNECTION['port'])
        db = client[self.database.__database_name__]
        db.authenticate(DATABASE_CONNECTION['user'], DATABASE_CONNECTION['password'])
        collection = db[DATABASE_CONNECTION['collection_rating']]
        collection.drop()
        client.close()

    def test_save_new_and_add_rating(self):
        """
        Tests all functionality of save_new() and add_rating().
        """
        edge_from = 4
        edge_to = 5
        rating = 2

        # test save_new on clean database
        self.database.save_new(edge_from, edge_to)
        self.assertTrue(self.check_edge_in_database(edge_from, edge_to, 0, 0))

        # test add_rating on 1 item database
        self.database.add_rating(edge_from, edge_to, rating)
        self.assertTrue(self.check_edge_in_database(edge_from, edge_to, rating, 1))

        # test save_new doesn't overwrite
        self.database.save_new(edge_from, edge_to)
        self.assertTrue(self.check_edge_in_database(edge_from, edge_to, rating, 1))

        edge_from_v2 = 8
        edge_to_v2 = 6

        # test save_new on non-empty database
        self.database.save_new(edge_from_v2, edge_to_v2)
        self.assertTrue(self.check_edge_in_database(edge_from, edge_to, rating, 1))
        self.assertTrue(self.check_edge_in_database(edge_from_v2, edge_to_v2, 0, 0))

        edge_from_v3 = 6
        edge_to_v3 = 6
        rating_v3 = 45

        # test add_rating on multiple item database
        self.database.add_rating(edge_from_v3, edge_to_v3, rating_v3)
        self.assertTrue(self.check_edge_in_database(edge_from, edge_to, rating, 1))
        self.assertTrue(self.check_edge_in_database(edge_from_v2, edge_to_v2, 0, 0))
        self.assertTrue(self.check_edge_in_database(edge_from_v3, edge_to_v3, rating_v3, 1))

    def test_load_methods(self):
        """
        Tests all functionality for get_average_rating(), get_amount_voted(), get_edge().
        Assumes the save methods are working correctly.
        """
        edge_from_v1 = 4
        edge_to_v1 = 8
        rating_v1 = 6
        self.database.save_new(edge_from_v1, edge_to_v1)

        # test for zero edge
        self.assertEqual(0.0, self.database.get_average_rating(edge_from_v1, edge_to_v1))
        self.assertEqual(0, self.database.get_amount_voted(edge_from_v1, edge_to_v1))
        self.assertTupleEqual((0, 0, 0), self.database.get_edge(edge_from_v1, edge_to_v1))

        # test for non existing edge
        self.assertEqual(0.0, self.database.get_average_rating(987, 1564))
        self.assertEqual(0, self.database.get_amount_voted(987, 1564))
        self.assertIsNone(self.database.get_edge(987, 1564))

        self.database.add_rating(edge_from_v1, edge_to_v1, rating_v1)

        # test for 1 added rating
        self.assertEqual(rating_v1, self.database.get_average_rating(edge_from_v1, edge_to_v1))
        self.assertEqual(1, self.database.get_amount_voted(edge_from_v1, edge_to_v1))
        self.assertTupleEqual((rating_v1, 1, rating_v1), self.database.get_edge(edge_from_v1, edge_to_v1))

    def test_bulk_methods(self):
        """
        Tests all functionality for save_new_list(), add_rating_list(), get_all_edges().
        """
        edge_from_v1 = 6
        edge_to_v1 = 7
        rating_v1 = 9

        edge_from_v2 = 9
        edge_to_v2 = 4
        rating_v2 = 5

        list_v1 = []
        list_v1.append((edge_from_v1, edge_to_v1, 0, 0))
        list_v1.append((edge_from_v2, edge_to_v2, 0, 0))

        self.database.save_new_list(list_v1)

        # check both edges in database
        self.assertTrue(self.check_edge_in_database(edge_from_v1, edge_to_v1, 0, 0))
        self.assertTrue(self.check_edge_in_database(edge_from_v2, edge_to_v2, 0, 0))

        list_v2 = []
        list_v2.append((edge_from_v1, edge_to_v1, rating_v1))
        list_v2.append((edge_from_v2, edge_to_v2, rating_v2))

        self.database.add_rating_list(list_v2)

        # check both edges in database
        self.assertTrue(self.check_edge_in_database(edge_from_v1, edge_to_v1, rating_v1, 1))
        self.assertTrue(self.check_edge_in_database(edge_from_v2, edge_to_v2, rating_v2, 1))

        all_edges = self.database.get_all_edges()

        # check tuples returned
        self.assertTupleEqual((rating_v1, 1, rating_v1), all_edges[edge_from_v1, edge_to_v1])
        self.assertTupleEqual((rating_v2, 1, rating_v2), all_edges[edge_from_v2, edge_to_v2])

    def check_edge_in_database(self, edge_from, edge_to, total_rating, amount_voted):
        """
        Utility method for direct checking if an edge is in the database.
        :return True if an edge with the given parameters is in the database, False otherwise
        """
        client = MongoClient(DATABASE_CONNECTION['host'], DATABASE_CONNECTION['port'])
        db = client[self.database.__database_name__]
        db.authenticate(DATABASE_CONNECTION['user'], DATABASE_CONNECTION['password'])
        collection = db[DATABASE_CONNECTION['collection_rating']]

        query = {DATABASE_SCHEME_EDGE_RATING['edge_from']: edge_from,
                 DATABASE_SCHEME_EDGE_RATING['edge_to']: edge_to,
                 DATABASE_SCHEME_EDGE_RATING['total_rating']: total_rating,
                 DATABASE_SCHEME_EDGE_RATING['amount_voted']: amount_voted}

        result = collection.find_one(query)
        client.close()

        if result is None:
            # nothing is found
            return False
        else:
            return True
