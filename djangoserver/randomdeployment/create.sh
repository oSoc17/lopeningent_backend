curl $(cat $1 | grep -v ^$ | awk '{print "https://www.openstreetmap.org/api/0.6/map?bbox="$1"%2C"$2"%2C"$3"%2C"$4;}') > mapdata.osm

(cd .. && python preprocess/main.py)
(cd ../dataonsteroids && tar -czvf data.tar.gz nodes.json ways.json)
(cd .. && python manage.py runserver 8000)
