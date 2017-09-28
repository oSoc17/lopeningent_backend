from django.http import HttpResponse, HttpResponseNotFound
from django.views.decorators.csrf import csrf_exempt
from server.database import get_poi_coords, get_poi_types
from server.config import route_poi
import json

@csrf_exempt
def get_coords(request):
    with open ('server/poi_coord_route.py') as f:
        lines = f.read().splitlines()
        
    route_poi = map(int, lines)
    tags = request.POST.getlist("tags")
    available_tags = get_poi_types()

    for tag in tags:
        if tag not in available_tags:
            return HttpResponseNotFound("One of your tags does not exist.")

    return HttpResponse(json.dumps({"coords": get_poi_coords(tags,route_poi)}))


@csrf_exempt
def get_types(request):
    return HttpResponse(json.dumps({"types": get_poi_types()}))