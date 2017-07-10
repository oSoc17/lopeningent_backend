import random


class RandomChooser(object):
    """
        Deprecated
    """
    def __init__(self):
        self.total = 0.0
        self.currentchoice = None

    def push(self, weight, item):
        if random.uniform(0.0, 1.0) < weight / (self.total + weight):
            self.currentchoice = item
        self.total += weight

    def pop(self):
        return self.currentchoice


def ground(visited_node_dict, end_node):
    """
        'grounds' a rod: it returns a list of nodes given a dict of nodes pointing to their precessor.

        For example:
        {2 : 1, 3 : 2, 5 : 3}, 5 -> [1, 2, 3, 5]
    """
    res = []
    curr = end_node
    while curr is not None:
        res.append(curr)
        curr = visited_node_dict.get(curr, None)
    return res[::-1]

def unground(visited_nodes):
    """
        Performs the inverse 'grounding' operation
    """
    res = {b : a for a, b in zip(visited_nodes, visited_nodes[1:])}
    return res
