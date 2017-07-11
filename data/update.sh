#!/bin/bash
# Author: Tim Baccaert <github.com/timaert>
# Version: 0.0.1

# These are parameters for OSM's bounding box of Ghent.
# The overpass-api is a third-party host for the latest OSM data.
url="http://overpass-api.de/api/map?bbox"
lon1=3.6400
lat1=50.9800
lon2=3.8400
lat2=51.1100

# This downloads the latest map data from OSM,
# the result is stored inside the raw_ghent.osm xml file.
#curl "$url=$lon1,$lat1,$lon2,$lat2" > raw_ghent.osm

# We're filtering out the ways which are suitable for pedestrians.
# https://wiki.openstreetmap.org/wiki/Pedestrian
highway_tags="pedestrian,footway,path,cylceway"
foot_tags="yes,designated"

osmosis \
	--read-xml raw_ghent.osm \
	--tag-filter accept-ways highway=$highway_tags \
	--tag-filter accept-ways foot=$foot_tags \
	--tag-filter reject-relations \
	--used-node \
	--write-xml ghent.osm

# Delete the raw file
#rm -f raw_ghent.osm

# Run the migration script to update the,
# data inside the postgresql database.
./migrate.py