class Node:

    def __init__(self, nid, coord):
        self.id = nid
        self.lat, self.lon = coord
        self.x = 0.0
        self.y = 0.0

    def __str(self):
        return "#{} ({}, {})".format(self.nid, self.coord[0], self.coord[1])