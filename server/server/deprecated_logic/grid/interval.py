from collections import namedtuple
from server.logic.grid.grid import GridBuilder


class Interval(namedtuple('Interval', 'minx miny maxx maxy owner')):
    """
        A class representing a twodimensional interval.

        Fields:
        minx, miny, maxx, maxy -- lower/upper x/y bound.
        owner -- extra metadata to recognise an interval on retrieval. Usually
                 contains a tuple of graph nodes (see Graph).
    """

    def __add__(self, other):
        """
            Join two intervals, creating the smallest rectangle that fully contains both of them.
        """
        return Interval(
            min(self.minx, other.minx),
            min(self.miny, other.miny),
            max(self.maxx, other.maxx),
            max(self.maxy, other.maxy),
            None)

    def into_grid(self, bin_size):
        """ Create a grid stretching over the interval, with the given bin size """
        width = int((self.maxx - self.minx) / bin_size + 1)
        height = int((self.maxy - self.miny) / bin_size + 1)
        min_x = (self.maxx + self.minx - width * bin_size) / 2.
        min_y = (self.maxy + self.miny - height * bin_size) / 2.
        return GridBuilder()\
            .with_size(width, height)\
            .with_binsize(bin_size)\
            .with_offset(min_x, min_y)\
            .create()


def into_interval(startnode, endnode, tolerance):
    """ Creates an interval that contains the two nodes with a certain tolerance """
    min_x = min(startnode.x, endnode.x) - tolerance
    min_y = min(startnode.y, endnode.y) - tolerance
    max_x = max(startnode.x, endnode.x) + tolerance
    max_y = max(startnode.y, endnode.y) + tolerance
    return Interval(min_x, min_y, max_x, max_y, (startnode.id, endnode.id))
