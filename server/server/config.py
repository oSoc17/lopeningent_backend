# Earth radius in km. Modifiability apparently includes moving to Jupiter.
EARTH_RADIUS = 6371

# Interval tolerance. Should be larger than the GPS accuracy.
TOLERANCE = 0.1

# Grid bin size
BINSIZE = 0.1

# Database connection parameters
DB_CONN = "host=localhost user=postgres password=idlab_lopeningent"

DEFAULT_ROUTING_CONFIG = {
    'max_length': 10.0,
    'min_length': 5.0,
    'poison_max_value': 120.0,
    'poison_max_distance': 2.0,
    'poison_min_value': 1.0,
    'cross_penalty': 100.0,
    'measure_length': 1.0
}

RETURN_ROUTING_CONFIG = DEFAULT_ROUTING_CONFIG
