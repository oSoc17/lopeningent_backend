import base64

def encode(integer):
    """
        Encodes an (very large) integer into a string.
    """
    hex = '{:x}'.format(integer)
    if len(hex) % 2 == 1:
        hex = '0' + hex
    encoded = base64.b64encode(bytearray.fromhex(hex))
    encoded = encoded.replace("/", "-")
    encoded = encoded.replace("+", "*")
    while encoded[-1] == '=':
        encoded = encoded[:-1]
    return encoded

def decode(string):
    """
        Decodes the string generated by encode back into an integer
    """
    string = string.replace("-", "/")
    string = string.replace("*", "+")
    string = string + '=' * ((3*len(string)) % 4)
    print string
    hex = base64.b64decode(string).encode('hex')
    return int(hex, 16)

def into_string(graph, rod):
    """
        Transforms a list of indices into a tag.
    """
    num = rod[0]
    res = [[i for i in graph.get_connids(prev)].index(index) for (prev, index) in zip(rod, rod[1:])]
    res = int("".join([str(i + 1) for i in res]))
    return encode(num + res*graph.largest)

def from_string(graph, string):
    """
        Creates a rod from a graph and tag.
    """
    num = decode(string)
    first = num % graph.largest
    res = [first]
    for offset in [int(i) for i in str(num / graph.largest)]:
        res.append([i for i in graph.get_connids(res[len(res) - 1])][offset - 1])
    return res