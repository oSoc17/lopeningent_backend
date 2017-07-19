from collections import namedtuple
from server.logic.distance.util import angle


class DirectionDict(object):
    def __init__(self):
        self.none = "none"
        self.forward = "forward"
        self.hasleft = "hasleft"
        self.hasright = "hasright"
        self.left = "left"
        self.right = "right"
        self.turn = "turnaround"


DIRECTION_DICTIONARY = DirectionDict()


class Direction(namedtuple('Direction', 'lat lon c')):
    """
        Class for serializing into the direction type
    """
    @staticmethod
    def from_index(graph, index, c):
        node = graph.get(index)
        return Direction(node.lat, node.lon, c)


def get_direction(node_angle, threshold, left, right):
    """
        Return right or left depending on the angle

        Function args:
        node_angle -- list of three points forming an angle.
        threshold -- minimum (-maximum) value of the sine of the angle
                     in order to count
        left, right -- return values in case of success
    """
    ag = angle(node_angle[0], node_angle[1], node_angle[2])
    if ag > threshold:
        return right
    elif ag < -threshold:
        return left
    else:
        return None

# TODO: add some comment please!
def into_directions(graph, nodelist, dirdict):
    """
        Annotate a nodelist with directions

        Returns: list of coordinates with direction metadata.
    """
    # Minimum sine of an angle to be considered a turn.
    threshold = 0.7


    iterator = zip(nodelist, nodelist[1:], nodelist[2:])
    res_list = [Direction.from_index(graph, nodelist[0], dirdict.none)]
    for id_prev, id_curr, id_next in iterator:
        c = None
        if id_prev == id_next:
            c = dirdict.turn
        elif len(graph.get_connids(id_curr)) > 2:

            # Check if the road makes a sharp turn.
            node_angle = [graph.get(id_prev), graph.get(
                id_curr), graph.get(id_next)]
            c = get_direction(node_angle, threshold,
                              dirdict.left, dirdict.right)

            # Check if the road could have made a sharp turn.
            glist = graph.get_connids(id_curr)
            while c is None and len(glist) > 0:
                id_poss_next = glist[0]
                glist = glist[1:]
                node_angle = [graph.get(id_prev), graph.get(
                    id_curr), graph.get(id_poss_next)]
                c = get_direction(node_angle, threshold,
                                  dirdict.hasleft, dirdict.hasright)
        if c is None:
            c = dirdict.none
        res_list.append(Direction.from_index(graph, id_curr, c))
    res_list.append(Direction.from_index(
        graph, nodelist[::-1][0], dirdict.none))

    last = dirdict.none
    final_list = []

    # Iterate over everything we found in reverse order, and apply the transfomation.
    # This code is a remnant of when forward was meant to create the message
    # "Take the third street on your left."
    res_list = res_list[::-1]
    for node in res_list:
        new_node = node
        if node.c == dirdict.left or node.c == dirdict.right or node.c == dirdict.turn:
            # Left/Right -> Left/Right
            last = node.c
        elif (node.c == dirdict.hasleft) or (node.c == dirdict.hasright):
            # Hasleft/Hasright -> Forward
            new_node = new_node._replace(c=dirdict.forward)
        else:
            # Other -> None
            new_node = new_node._replace(c=dirdict.none)
        final_list.append(new_node)
    final_list = final_list[::-1]
    return final_list
