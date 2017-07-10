import json
import time


def _compile(struct):
    if hasattr(struct, '_asdict') or isinstance(struct, dict):
        if hasattr(struct, '_asdict'):
            struct = struct._asdict()
        return {key: _compile(arg) for key, arg in struct.items()}
    elif isinstance(struct, (list, tuple)):
        return [_compile(arg) for arg in struct]
    else:
        return struct


def into_json(struct):
    """
        Transforms a named tuple into a json object in a nice way.
    """
    return json.dumps(_compile(struct), indent=2)


def time_fn(lbd):
    """
        Times an expression.

        If you need to time the operation:
            result = heavy_function()
        Then just write:
            result = time_fn(lambda : heavy_function())
    """
    start = time.time()
    res = lbd()
    end = time.time()
    print("TIME: %s" % (end - start))
    return res
