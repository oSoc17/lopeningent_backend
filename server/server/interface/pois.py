from django.http import HttpResponse, HttpResponseNotFound
from django.views.decorators.csrf import csrf_exempt
from server.database import get_poi_coords, get_poi_types
from server.config import route_poi
import json,logging,ast
from collections import defaultdict
import cPickle as pic

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

@csrf_exempt
def save_coords(request):
  try:
      ele = request.POST.getlist("elements")
      name = str(request.POST.get("name"))
      if name.isspace():
        logging.error('saving POis failed since there is no name or there is space in the name',exc_info=True)
        return HttpResponse("Json not Saved")

      else:
        filename = "/home/kthiruko/newbackend_rust/data/poisets/%s.json" %name
        
        ## I have no idea whyy we get 2 square brackets while writing to a json file so did some stuff to make it right
        json_data = {"elements" : map (ast.literal_eval , ele) , "name": name}
        struct_element = map (ast.literal_eval , ele)
        d = defaultdict(list)
        d["elements"] = struct_element
        d["name"]=name
        json_d = json.dumps(d)
        data = json.loads(json_d)
        temp = data["elements"]
        data["elements"] = temp[0]
        # what1 = json.dumps(what)
        # print what1

        with open(filename, 'w') as outfile:
            json.dump(data,outfile)
        
        return HttpResponse("Json Saved")
      
  except:
    logging.error('saving POis failed',exc_info=True)
    return HttpResponse("Json not Saved")