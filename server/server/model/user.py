import datetime
import json

class User:

    def __init__(self, uid, avg_speed, avg_heartrate, avg_distance,tot_distance,tot_duration,avg_duration,runs,edit_time ):
        self.uid = str(uid)
        self.avg_speed = float(avg_speed)
        self.avg_heartrate = int(avg_heartrate)
        self.avg_distance = int(avg_distance)
        self.tot_duration = int(tot_duration)
        self.avg_duration = int(avg_duration)
        self.tot_distance = int(tot_distance)
        self.runs = int(runs)
        self.edit_time = long(edit_time )


    def __str__(self):
        return "#{}/ {}/ {}/ {}/ {}/ {}/ {}/ {}".format(self.uid, self.avg_speed,self.avg_heartrate,self.tot_distance,self.tot_duration,self.tot_distance,self.avg_duration,self.runs)

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__,
            sort_keys=True, indent=9)