#!/bin/bash
# Author: Tim Baccaert <github.com/timaert>

fullpath=$(dirname "$0")

# These are parameters for OSM's bounding box of Ghent.
# The overpass-api is a third-party host for the latest OSM data.
url="http://overpass-api.de/api/map?bbox"
lon1=3.6400
lat1=50.9800
lon2=3.8400
lat2=51.1100

# This downloads the latest map data from OSM,
# the result is stored inside the raw_ghent.osm xml file.
curl "$url=$lon1,$lat1,$lon2,$lat2" -o $fullpath/raw_set.osm

# We're filtering out the ways which are suitable for pedestrians.
# https://wiki.openstreetmap.org/wiki/Pedestrian

# Put your tags under --tag-filter accept-ways

osmosis \
	--read-xml $fullpath/raw_set.osm \
	--tag-filter accept-nodes waterway=* \
	--tag-filter reject-ways \
    --tag-filter reject-relations \
	--used-node \
	--write-xml $fullpath/set.osm