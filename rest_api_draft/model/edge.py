class Edge:

    def __init__(self, from_id, to_id, rating, tags):
        self.from_id = int(from_id)
        self.to_id = int(to_id)
        self.rating = float(rating)
        self.tags = list(tags)

    def __str__(self):
        return "#{} -> #{}".format(self.from_id, self.to_id)
