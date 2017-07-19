from django.http import HttpResponse, HttpResponseNotFound, HttpResponseBadRequest
from django.views.decorators.csrf import csrf_exempt

"""
    Responds with a route starting and ending in a certain coordinate

    Query args:
    lat -- coordinate latitude
    lon -- coordinate longitude
    tags -- the requested POI's
    type -- the response type. See the geojson.respond_path function for more details
"""
@csrf_exempt
def generate(request):
    if (str(request.POST.get('android_token'))== "1223"): # this should be a method of actually checking with firebase
        lat = request.POST.get('lat')
        lon = request.POST.get('lon')
        tags = request.POST.getlist('tags')
        type = request.POST.get('type')

        #route_from_coord method called - similar to current situation


    else:
        print "You don't have access to this api from outside the android app."

"""
    Responds with a route leading the user back to his starting point.

    Query args:
    rid -- the route that the user used to run.
    lon, lat -- position of the user.
    distance -- The preferred distance to the starting point.
    tags -- the requested POI's

"""
@csrf_exempt
def return_home(request):
    if (str(request.POST.get('android_token'))== "1223"):

        # Get path from request tag
        tags = request.POST.getlist('tags')

    else:
        print "You don't have access to this api from outside the android app."

"""
    Adds the rating to all edges in a route, and saves it both in the structure and in the database.

    Query args:
    rid -- the id for the rated route
    rating -- a float between 0 and 5
"""
@csrf_exempt
def rate_route(request):
    if (str(request.POST.get('android_token'))== "1223"):


        tag = request.POST.get('tag')
        rating = float(request.POST.get('rating'))
        return HttpResponse('')

    else:
        print "You don't have access to this api from outside the android app."