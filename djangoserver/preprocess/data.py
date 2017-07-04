from collections import namedtuple


# There are 3 ways to represent XML/JSON data in python: data classes, dicts and namedtuple's.
# Both data classes and dicts use to much memory, so I've opted for namedtuple's.

class Node(namedtuple('Node', 'id lat lon water park tags')):
    """
    id      integer
    lat     float
    lon     float
    water   int (0 or 1)
    park    int (0 or 1)
    tags    dict
    """
    pass


class Way(namedtuple('Way', 'id nodes tags')):
    """
    id      integer
    nodes   list
    tags    dict
    """
    pass


class Relation(namedtuple('Relation', 'id members tags')):
    """
    id          integer
    members     list of tuples ('type', 'ref', 'role') with data types (string, int, string)
    tags        dict
    """
    pass
