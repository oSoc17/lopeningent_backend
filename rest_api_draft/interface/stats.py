from django.http import HttpResponse
from django.http import HttpResponseNotFound
from django.views.decorators.csrf import csrf_exempt
import json
from rest_api_draft.database import get_stats_user,update_stats_user



@csrf_exempt
def get_stats_from_id(request):
    if (str(request.POST.get('android_token'))== "1223"):
        userid = request.POST.get('userid')
        print "Hello " + str(userid)

        result = get_stats_user(userid)
        resp ={}
        resp['message'] = 'no error'
        json.dumps(resp)
        resp['values'] = result
        return HttpResponse(resp, content_type="application/json")

        # put the stats in a response
    else:
        print "You don't have access to this api from outside the android app."
        resp ={}
        resp['message'] = 'acces denied'
        resp['values'] = ''
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


        update_stats_user(userid,avg_speed, avg_heartrate, avg_distance,tot_distance,tot_duration,avg_duration,runs)
        # send response saying db is updated


        print "Hello " + str(userid)

    else:
        print "You don't have access to this api from outside the android app."