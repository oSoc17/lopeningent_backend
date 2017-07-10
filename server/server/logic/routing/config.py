from collections import namedtuple
import json


class RoutingConfig(namedtuple('RoutingConfig',
                               'min_length max_length \
                                poison_min_value poison_max_value \
                                poison_max_distance cross_penalty \
                                measure_length measure_highway \
                                measure_rating measure_sheep \
                                measure_water measure_park')):
                                
    """
        The class containing all possible config options for routing.

        Keep it up to date with server.config.DEFAULT_ROUTING_CONFIG
    """
    pass


def from_dict(default, dictionary):
    """ Generates a routing configuration from a dictionary """
    res = {key: json.loads(dictionary.get(key, str(val)))
           for key, val in default.items()}
    return RoutingConfig(**res)
