from django.http import HttpResponse
from django.http import HttpResponseNotFound
from django.views.decorators.csrf import csrf_exempt
import json

from server.model.user import User
from server.database import get_stats_user,update_stats_user


@csrf_exempt
def get_stats_from_id(request):
    if (str(request.POST.get('android_token'))== "1223"):
        userid = request.POST.get('userid')
        print "Hello " + str(userid)

        result = get_stats_user(userid)
        if  result==None:
            resp = {'message': 'error', 'values': object}
            resp['values'] = ''
        else:
            resp = {'message': 'no error', 'values': object}
            resp['values'] = (result)

        return HttpResponse(json.dumps(resp), content_type="application/json")

        # put the stats in a response
    else:
        print "You don't have access to this api from outside the android app."
        resp ={'message': 'acces denied','values' : None}

        return HttpResponse(json.dumps(resp), content_type="application/json")

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

        requestedUser = User(userid, avg_speed, avg_heartrate, avg_distance, tot_distance, tot_duration, avg_duration, runs)
        update_stats_user(requestedUser)
        # send response saying db is updated


        print "Hello " + str(userid)

    else:
        print "You don't have access to this api from outside the android app."