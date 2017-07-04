#!/usr/bin/env python
import csv
import json
import math

class parseDump:
    def __init__(self):
        self.routes = {}

    def read(self):
        output_filename = "running_routes.json"

        with open("dump_running_route.csv", "rb") as csv_dump:
            routereader = csv.reader(csv_dump, delimiter=';')

            # Skip header
            next(routereader, None)
            idIterator = 1
            for row in routereader:
                row[2] = self.decode_polyline(row[2])
                self.routes[int(idIterator)] = []

                # score is a float between 0 and 1
                # length is a float in km
                self.routes[int(idIterator)].append({
                    'route_name': row[1],
                    'coordinates': row[2],
                    'score': float(row[3]),
                    'view_count': int(row[5]),
                    'length': float(row[6])/1000.0
                })

                idIterator = idIterator + 1

            print str(idIterator) + " routes retrieved"

            with open(output_filename, "w") as jsonfile:
                json.dump(self.routes, jsonfile)

            print str(idIterator) + " routes written to " + output_filename


    # Decode Google polyline string format to array of latlng coordinates
    def decode_polyline(self, polyline):
        points = []
        index = lat = lng = 0

        while index < len(polyline):
            result = 1
            shift = 0
            while True:
                b = ord(polyline[index]) - 63 - 1
                index += 1
                result += b << shift
                shift += 5
                if b < 0x1f:
                    break
            lat += (~result >> 1) if (result & 1) != 0 else (result >> 1)

            result = 1
            shift = 0
            while True:
                b = ord(polyline[index]) - 63 - 1
                index += 1
                result += b << shift
                shift += 5
                if b < 0x1f:
                    break
            lng += ~(result >> 1) if (result & 1) != 0 else (result >> 1)

            points.append({"lat": "%.7f" % (lat * 1e-5), "lon": "%.7f" % (lng * 1e-5)})

        return points

if __name__ == "__main__":
    reader = parseDump()
    reader.read()
