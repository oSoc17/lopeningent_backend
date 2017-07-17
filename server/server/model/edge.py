class Edge:

    def __init__(self, id, to, distance=0.0, modifier=0.0, poison=1.0):
        # Data used in Rust struct
        self.id = int(id)
        self.distance = float(distance)
        self.modifier = float(modifier)
        self.poison = float(poison)
        self.to = int(to)

        # Data used to compute modifier
        self._rating = None
        self._tags = None

    def __str__(self):
        return "#{} -> #{} ({})".format(self.id, self.to, self.distance)

    def set_modifier_data(self, rating, tags):
        self._rating = rating
        self._tags = list(tags)

    def into_c(self):
        return (self.id, self.distance, self.modifier, self.poison, self.to)
