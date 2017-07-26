class Edge:

    def __init__(self, id, distance, modifier, poison, to):
        # Data used in Rust struct
        self.id = int(id)
        self.distance = float(distance)
        self.modifier = float(modifier)
        self.poison = float(poison)
        self.to = int(to)

        # Data used to compute modifier
        self.rating = None
        self.tags = None

    def __str__(self):
        return "#{} -> #{} dist({}) mod({}) poison({})".format(self.id, self.to, 
            self.distance, self.modifier, self.poison)

    def __repr__(self):
        return "#{} -> #{} dist({}) mod({}) poison({})".format(self.id, self.to, 
            self.distance, self.modifier, self.poison)

    def set_modifier_data(self, rating, tags):
        self.rating = rating
        self.tags = list(tags)

    def into_c(self):
        return (self.id, self.distance, self.modifier, self.poison, self.to)