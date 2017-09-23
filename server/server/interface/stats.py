import os

from django.http import HttpResponse
from django.http import HttpResponseNotFound
from django.views.decorators.csrf import csrf_exempt
import json, logging

from server.model.user import User
from server.database import get_stats_user,update_stats_user
import firebase_admin
from firebase_admin import credentials
from firebase_admin import auth

# firebase authentication
cred = credentials.Certificate(os.path.join(os.path.dirname(__file__), '../firebase/firebase_auth.json'))
default_app = firebase_admin.initialize_app(cred)


@csrf_exempt
def get_stats_from_id(request):
    logging.info("STATS: received get_stats_from_id request")
    uid = ''
    try:
        logging.debug("STATS: get_stats_from_id request header was: %s", request)
        decoded_token = auth.verify_id_token(str(request.POST.get('android_token')))
        userid = decoded_token['uid']
        logging.info("STATS: user with uid %s authenticated", str(userid))
        result = get_stats_user(userid)
        if result == None:
            resp = {'message': 'error', 'values': object}
            resp['values'] = {'edit_time': 0}
        else:
            resp = {'message': 'no error', 'values': object}
            resp['values'] = (result)

        logging.debug("STATS: response was %s:", json.dumps(resp))
        return HttpResponse(json.dumps(resp), content_type="application/json")
    except ValueError:
        logging.error("STATS: You don't have access to this api from outside the android app/Wrong Firebase token")
        resp ={'message': 'acces denied','values' : None}
        logging.error("STATS: response that caused error was: %s", json.dumps(res))
        return HttpResponse(json.dumps(resp), content_type="application/json")



@csrf_exempt
def post_stats_from_id(request):
    logging.info("STATS: received post_stats_from_id request")
    uid = ''
    try:
        decoded_token = auth.verify_id_token(str(request.POST.get('android_token')))
        userid = decoded_token['uid']
        avg_speed = request.POST.get('avg_speed')
        avg_heartrate = request.POST.get('avg_heartrate')
        avg_distance = request.POST.get('avg_distance')
        tot_distance = request.POST.get('tot_distance')
        tot_duration = request.POST.get('tot_duration')
        avg_duration = request.POST.get('avg_duration')
        runs = request.POST.get('runs')
        edit_time = request.POST.get('edit_time')
        requestedUser = User(userid, avg_speed, avg_heartrate, avg_distance, tot_distance, tot_duration, avg_duration, runs,edit_time)
        logging.debug("STATS: json: %s", requestedUser.toJSON())

        updated = update_stats_user(requestedUser)
        if updated:
            resp = {'message': 'no error', 'values': "updated"}

        else:
            resp = {'message': 'error', 'values': "something went wrong when updating/inserting"}

        logging.debug("STATS: responded: %s", json.dumps(resp))
        return HttpResponse(json.dumps(resp), content_type="application/json")


    except ValueError:
        logging.error("STATS: You don't have access to this api from outside the android app/Wrong Firebase token")
        resp = {'message': 'acces denied', 'values': None}
        logging.error("STATS: response that caused error was: %s", json.dumps(resp))
        return HttpResponse(json.dumps(resp), content_type="application/json")
