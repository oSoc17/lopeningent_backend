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
curl -sSf "$url=$lon1,$lat1,$lon2,$lat2" -o $fullpath/raw_ghent.osm

# We're filtering out the ways which are suitable for pedestrians.
# https://wiki.openstreetmap.org/wiki/Pedestrian
highway_tags="motorway,motorway_link,trunk,primary,secondary,trunk_link,primary_link"

osmosis \
	--read-xml $fullpath/raw_ghent.osm \
	--tag-filter accept-ways highway=* \
	--tag-filter reject-ways highway=$highway_tags \
	--tag-filter reject-relations \
	--used-node \
	--write-xml $fullpath/ghent.osm

# Delete the raw file
rm -f $fullpath/raw_ghent.osm

# Run the migration script to update the,
# data inside the postgresql database.
$fullpath/migrate.py
