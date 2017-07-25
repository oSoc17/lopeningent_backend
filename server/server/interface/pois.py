from django.http import HttpResponse, HttpResponseNotFound
from django.views.decorators.csrf import csrf_exempt
from server.database import get_poi_coords, get_poi_types
import json


@csrf_exempt
def get_coords(request):
    tags = request.POST.getlist("tags")
    available_tags = get_poi_types()

    for tag in tags:
        if tag not in available_tags:
            return HttpResponseNotFound("One of your tags does not exist.")

    return HttpResponse(json.dumps({"coords": get_poi_coords(tags)}))


@csrf_exempt
def get_types(request):
    return HttpResponse(json.dumps({"types": get_poi_types()}))