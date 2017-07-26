from xml.etree.cElementTree import iterparse
import json

def load_osm(osm_file):
    nodes = dict()

    for event, elem in iterparse(osm_file, events=("start", "end")):
        # Whenever the iterator encounters an opening tag
        if event == "start":
            if elem.tag == "node":
                curr_id = int(elem.attrib["id"])
                lat = float(elem.attrib["lat"])
                lon = float(elem.attrib["lon"])
                curr_elem = (lat, lon)

        # Whenever the iterator encounters a closing tag
        elif event == "end":
            if elem.tag == "node":
                nodes[curr_id] = curr_elem

    return nodes

def convert_to_json(node_dict):
    structure = { "name": "park", "elements": list() }

    for coord in node_dict.values():
        structure["elements"].append({
            "name": "park",
            "description": "It's a park!",
            "lat": coord[0], 
            "lon": coord[1]
        })
        
    with open("park.json", "w") as f:
        json.dump(structure, f, indent=4)

if __name__ == "__main__":
    convert_to_json(load_osm("set.osm"))