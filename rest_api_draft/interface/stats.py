from django.http import HttpResponse, HttpResponseNotFound, HttpResponseBadRequest
from django.views.decorators.csrf import csrf_exempt


#just a general feel of execution, will update later
@csrf_exempt
def get_stats_from_id(request):
    if (str(request.POST.get('android_token'))== "1223"):
        userid = request.POST.get('userid')
        print "Hello " + str(userid)

        # acces db and get stats based on id

        # put the stats in a response
    else:
        print "You don't have access to this api from outside the android app."

@csrf_exempt
def post_stats_from_id(request):
    if (str(request.POST.get('android_token'))== "1223"):
        userid = request.POST.get('userid')
        avg_speed = request.POST.get('avg_speed')
        avg_heartrate = request.POST.get('avg_heartrate')
        avg_distance = request.POST.get('avg_distance')
        tot_distance = request.POST.get('tot_distance')
        tot_duration = request.POST.get('tot_duration')
        avg_duration = request.POST.get('avg_duration')
        runs = request.POST.get('runs')


        # acces db and update stats

        # send response saying db is updated


        print "Hello " + str(userid)

    else:
        print "You don't have access to this api from outside the android app."