# Earth radius in km. Modifiability apparently includes moving to Jupiter.
EARTH_RADIUS = 6371

# Interval tolerance. Should be larger than the GPS accuracy.
TOLERANCE = 0.1

# Grid bin size
BINSIZE = 0.1

# Database connection parameters
DB_CONN = "dbname=lopeningent host=localhost user=postgres password=Q4a'30h=3*7hyg&ZKdR(4(6oQhFK>f6'`>L)UqhiZ$&aCvRGWlXoN*0o@M?IiDO"

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