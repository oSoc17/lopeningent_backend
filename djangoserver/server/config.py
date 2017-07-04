from subprocess import call

# Earth radius in km. Modifiability apparently includes moving to Jupiter.
EARTH_RADIUS = 6371

# Interval tolerance. Should be larger than the GPS accuracy.
TOLERANCE = 0.1

# Grid bin size
BINSIZE = 0.1

# Unzip the data
call("(cd dataonsteroids && tar -xzvf data.tar.gz)", shell=True)

# Map with road data
GRAPH_SOURCE = "dataonsteroids"

# Database connection parameters
DATABASE_CONNECTION = {
    'user': 'server',
    'password': 'dynamicrunninginghentGraph',
    'host': 'groep16.cammaert.me',
    'port': 27017,
    'database': 'edges',
    'collection_rating': 'rating'
}

# Database scheme edge rating
DATABASE_SCHEME_EDGE_RATING = {
    'edge_from': 'edge_from',
    'edge_to': 'edge_to',
    'total_rating': 'total_rating',
    'amount_voted': 'amount voted'
}

DEFAULT_ROUTING_CONFIG = {
    'min_length': 5.0,
    'max_length': 10.0,
    'poison_max_value': 120.0,
    'poison_max_distance': 2.0,
    'poison_min_value': 1.0,
    'cross_penalty': 100.0,
    'measure_length': 1.0,
    'measure_highway': 1.0,
    'measure_rating' : 0.0,
    'measure_sheep' : 0.0,
    'measure_water' : 0.0,
    'measure_park' : 0.0,
}

RETURN_ROUTING_CONFIG = DEFAULT_ROUTING_CONFIG

# TODO Configure with proper values
HIGHWAY_WHITELIST = {
    'proposed': 7,
    'primary': 50,
    'pedestrian': 1,
    'bridleway': 1,
    'secondary_link': 500,
    'tertiary': 7,
    'primary_link': 500,
    'service': 20,
    'residential': 1,
    'motorway_link': float('inf'),
    'cycleway': 4,
    'platform': 100,
    'secondary': 50,
    'living_street': 2,
    'track': 7,
    'motorway': float('inf'),
    'construction': 15,
    'tertiary_link': 7,
    'trunk': 500,
    'path': 5,
    'trunk_link': 500,
    'footway': 1,
    'unclassified': 4,
    'bus_stop': 15,
    'steps': 3,
    'road': 100,
    'default': 10
}
