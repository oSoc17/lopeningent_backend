from pymongo import MongoClient
from server.config import DATABASE_CONNECTION
from server.config import DATABASE_SCHEME_EDGE_RATING


class EdgeDatabase(object):
    """
    Class responsible for saving and loading statistics for edges.
    Every method in this class is sets up it's own connection to the database.
    As a result, if you wish to load a lot of data,
    it is better to use "get_all_edges()" instead of iterating over "get_edge()".

    Usage example:
        database = EdgeDatabase()
        database.add_rating(edge_from, edge_to, 6)
    """
    def __init__(self):
        self.__collection__ = None
        self.__client__ = None
        # Database name is the only connection parameter that has a class variable.
        # This is so we are able to set it to a test database for tests.
        self.__database_name__ = DATABASE_CONNECTION['database']

    def save_new(self, edge_from, edge_to, total_rating=0, amount_voted=0):
        """
        Saves a new edge into the database.
        If a node with the same ID's (in the same order) already exists, it will NOT be overwritten.
        If you wish to initialize a large amount of edges, consider using 'add_rating_list()' with the rating set to 0.
        For a bulk version: see 'save_new_list()'
        :param edge_from: ID of node on one side.
        :param edge_to: ID of node on the other side.
        :param total_rating: rating of this edge (default 0).
        :param amount_voted: amount of people that voted for this edge (default 0).
        :return: Nothing.
        """
        edge_list = []
        edge_list.append((edge_from, edge_to, total_rating, amount_voted))
        self.save_new_list(edge_list)

    def save_new_list(self, edge_list):
        """
        Bulk version of 'save_new()'.
        Note: this bulk version has no default values.
        :param edge_list: List of tuples of the type (edge_from, edge_to, total_rating, amount_voted)
        :return: Nothing.
        """
        self.__connect__()
        for edge_from, edge_to, total_rating, amount_voted in edge_list:
            previous_document = self.__collection__.find_one(self.__make_query__(edge_from, edge_to))

            if previous_document is None:
                # only insert of not yet present
                document = {DATABASE_SCHEME_EDGE_RATING['edge_from']: edge_from,
                            DATABASE_SCHEME_EDGE_RATING['edge_to']: edge_to,
                            DATABASE_SCHEME_EDGE_RATING['total_rating']: total_rating,
                            DATABASE_SCHEME_EDGE_RATING['amount_voted']: amount_voted}
                self.__collection__.insert_one(document)
        self.__disconnect__()

    def add_rating(self, edge_from, edge_to, rating):
        """
        Adds a rating to an edge in the database. If an edge does not exist, it is created.
        For a bulk version: see 'add_rating_list()'
        :param edge_from: ID of node on one side.
        :param edge_to: ID of node on the other side.
        :param rating: rating to add to this edge total. Type: anything that can be added to a number.
        :return: Nothing
        """
        edge_list = []
        edge_list.append((edge_from, edge_to, rating))
        self.add_rating_list(edge_list)

    def add_rating_list(self, edge_list):
        """
        Bulk version of 'add_rating()'
        :param edge_list: List of tuples of the type (edge_from, edge_to, rating)
        :return: Nothing
        """
        self.__connect__()
        for edge_from, edge_to, rating in edge_list:
            previous_document = self.__collection__.find_one_and_delete(self.__make_query__(edge_from, edge_to))

            new_rating = rating
            amount_voted = 1

            if previous_document is not None:
                new_rating += previous_document[DATABASE_SCHEME_EDGE_RATING['total_rating']]
                amount_voted += previous_document[DATABASE_SCHEME_EDGE_RATING['amount_voted']]

            document = {DATABASE_SCHEME_EDGE_RATING['edge_from']: edge_from,
                        DATABASE_SCHEME_EDGE_RATING['edge_to']: edge_to,
                        DATABASE_SCHEME_EDGE_RATING['total_rating']: new_rating,
                        DATABASE_SCHEME_EDGE_RATING['amount_voted']: amount_voted}

            self.__collection__.insert_one(document)
        self.__disconnect__()

    def get_average_rating(self, edge_from, edge_to):
        """
        Looks up an edge in the database and returns the average rating for it.
        :param edge_from: ID of node on one side.
        :param edge_to: ID of node on the other side.
        :return: average rating for that node. 0 if the edge was not found or has no rating.
        """
        self.__connect__()
        document = self.__collection__.find_one(self.__make_query__(edge_from, edge_to))
        self.__disconnect__()

        if document is None or document[DATABASE_SCHEME_EDGE_RATING['amount_voted']] == 0:
            # check for division by zero
            return 0.0
        else:
            return document[DATABASE_SCHEME_EDGE_RATING['total_rating']] / document[
                DATABASE_SCHEME_EDGE_RATING['amount_voted']]

    def get_amount_voted(self, edge_from, edge_to):
        """
        Looks up an edge in the database and returns the amount of people that voted for it.
        :param edge_from: ID of node on one side.
        :param edge_to: ID of node on the other side.
        :return: average rating for that node. 0 if the edge was not found or has no votes.
        """
        self.__connect__()
        document = self.__collection__.find_one(self.__make_query__(edge_from, edge_to))
        self.__disconnect__()

        if document is None:
            return 0
        else:
            return document[DATABASE_SCHEME_EDGE_RATING['amount_voted']]

    def get_edge(self, edge_from, edge_to):
        """
        Looks up an edge in the database and returns all the data on it as a tupple:
        (total_rating, amount_voted, average_rating).
        :param edge_from: ID of node on one side.
        :param edge_to: ID of node on the other side.
        :return: (total_rating, amount_voted, average_rating), None if no edge is found.
        """
        self.__connect__()
        document = self.__collection__.find_one(self.__make_query__(edge_from, edge_to))
        self.__disconnect__()

        if document is None:
            return None
        else:
            total_rating = document[DATABASE_SCHEME_EDGE_RATING['total_rating']]
            amount_voted = document[DATABASE_SCHEME_EDGE_RATING['amount_voted']]
            if amount_voted == 0:
                average_rating = 0
            else:
                average_rating = total_rating / amount_voted
            return total_rating, amount_voted, average_rating

    def get_or_insert_edge(self, edge_from, edge_to, total_rating=0, amount_voted=0):
        """
        Retrieves an edge from the database. If the edge is not present, an new one will be created.
        :param edge_from: ID of node on one side.
        :param edge_to: ID of node on the other side.
        :param total_rating: Total rating to insert of new edge has to be created.
        :param amount_voted: Amount voted to insert of new edge has to be created.
        :return: (total_rating, amount_voted, average_rating) of the requested edge.
        """
        edge = self.get_edge(edge_from, edge_to)
        if edge is None:
            self.save_new(edge_from, edge_to, total_rating, amount_voted)
            if amount_voted == 0:
                average_rating = 0
            else:
                average_rating = total_rating / amount_voted
            edge = (total_rating, amount_voted, average_rating)
        return edge

    def get_all_edges(self):
        """
        Retrieves all edges in the database and returns them in a dict with key (edge_from, edge_to)
        and values (total_rating, amount_voted, average_rating).
        """
        result = {}

        self.__connect__()
        for document in self.__collection__.find():
            edge_from = document[DATABASE_SCHEME_EDGE_RATING['edge_from']]
            edge_to = document[DATABASE_SCHEME_EDGE_RATING['edge_to']]
            total_rating = document[DATABASE_SCHEME_EDGE_RATING['total_rating']]
            amount_voted = document[DATABASE_SCHEME_EDGE_RATING['amount_voted']]
            if amount_voted == 0:
                average_rating = 0
            else:
                average_rating = total_rating / amount_voted
            result[edge_from, edge_to] = (total_rating, amount_voted, average_rating)
        self.__disconnect__()

        return result

    def __connect__(self):
        """Connects to the database. Should be called before any interaction with the database."""
        self.__client__ = MongoClient(DATABASE_CONNECTION['host'], DATABASE_CONNECTION['port'])
        db = self.__client__[self.__database_name__]
        db.authenticate(DATABASE_CONNECTION['user'], DATABASE_CONNECTION['password'])
        self.__collection__ = db[DATABASE_CONNECTION['collection_rating']]

    def __disconnect__(self):
        """Cleans up database resources."""
        self.__client__.close()

    def __make_query__(self, edge_from, edge_to):
        """Little convenience method."""
        return {DATABASE_SCHEME_EDGE_RATING['edge_from']: edge_from,
                DATABASE_SCHEME_EDGE_RATING['edge_to']: edge_to}
