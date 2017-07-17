from collections import namedtuple
import json


class RoutingConfig(namedtuple('RoutingConfig', 
    'measure_length max_length min_length')):
                                
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
