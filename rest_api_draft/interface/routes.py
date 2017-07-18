from django.http import HttpResponse, HttpResponseNotFound, HttpResponseBadRequest
from django.views.decorators.csrf import csrf_exempt


#just a general feel of execution, will get updated later
@csrf_exempt
def generate(request):
    if (str(request.POST.get('android_token'))== "1223"): # this should be a method of actually checking with firebase
        lat = request.POST.get('lat')
        lon = request.POST.get('lon')

        # also get parameters like POI

        #route_from_coord method called


    else:
        print "You don't have access to this api from outside the android app."

@csrf_exempt
def return_home(request):
    if (str(request.POST.get('android_token'))== "1223"):
        print "lol"
    else:
        print "You don't have access to this api from outside the android app."


@csrf_exempt
def rate_route(request):
    if (str(request.POST.get('android_token'))== "1223"):
        """
            Adds the rating to all edges in a route, and saves it both in the structure and in the database.

            Query args:
            tag -- the tag of the route you want to rate
            rating -- a float between 0 and 5
        """

        tag = request.POST.get('tag')
        rating = float(request.POST.get('rating'))
        return HttpResponse('')

    else:
        print "You don't have access to this api from outside the android app."