class Node:

    def __init__(self, id, lat, lon, x=0.0, y=0.0):
        self.id = id
        self.lat = lat
        self.lon = lon
        self.x = x
        self.y = y

    def __str__(self):
        return "#{} lat/lon({}, {}) x/y({}, {})".format(self.id, self.lat, 
            self.lon, self.x, self.y)

    def __repr__(self):
        return "#{} lat/lon({}, {}) x/y({}, {})".format(self.id, self.lat, 
            self.lon, self.x, self.y)

    def into_c(self):
        return (self.id, self.lat, self.lon, self.x, self.y)