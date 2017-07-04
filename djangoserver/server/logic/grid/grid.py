from server.logic.projection.util import Coordinate

# A class to build a Grid


class GridBuilder(object):
    """
        A class meant for building a grid.
    """

    def __init__(self):
        """
            Create a new instance
        """
        self.width = None
        self.height = None
        self.binsize = None
        self.min_x = None
        self.min_y = None

    def with_size(self, width, height):
        """
            Sets the grid size
        """
        self.width = width
        self.height = height
        return self

    def with_offset(self, min_x, min_y):
        """
            Sets the grid offset
        """
        self.min_x = min_x
        self.min_y = min_y
        return self

    def with_binsize(self, binsize):
        """
            sets the bin size.
        """
        self.binsize = binsize
        return self

    def create(self):
        """
            Creates the actual grid.
        """
        return Grid(self)


class Grid(object):
    """
        A class representing a spatial bucket structure.
    """

    def __init__(self, context):
        """
            Initialisation. Do not use directly, but use a GridBuilder (the `context`
            argument) instead.
        """
        self.width = context.width
        self.height = context.height
        self.min_x = context.min_x
        self.min_y = context.min_y
        self.binsize = context.binsize
        self.data = [[[] for _ in xrange(self.width)]
                     for _ in xrange(self.height)]

    def get_xy(self, coord):
        """
            Returns an integer tuple containing the two indices to access a cell.
        """
        local_x = coord.x
        local_y = coord.y
        local_offset_x = int((local_x - self.min_x) / self.binsize)
        local_offset_y = int((local_y - self.min_y) / self.binsize)
        return (local_offset_x, local_offset_y)

    def get(self, coord):
        """
            Returns the content of the cell containing the coordinate
        """
        (local_offset_x, local_offset_y) = self.get_xy(coord)
        if self.inside(local_offset_x, local_offset_y):
            return self.data[local_offset_y][local_offset_x]
        return []

    def inside(self, x, y):
        """
            Returns whether the indices are not out of bounds.
        """
        return x >= 0 and x < self.width and y >= 0 and y < self.height

    def add_interval(self, interval):
        """
            Adds the owner of an interval to any cell overlapping with the interval.
        """
        if interval.owner is None:
            return
        (min_x, min_y) = self.get_xy(Coordinate(interval.minx, interval.miny))
        (max_x, max_y) = self.get_xy(Coordinate(interval.maxx, interval.maxy))
        for x in xrange(min_x, max_x + 1):
            for y in xrange(min_y, max_y + 1):
                if self.inside(x, y):
                    self.data[y][x].append(interval.owner)
