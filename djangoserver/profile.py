import cProfile
from django.test.client import RequestFactory
import time
import server.interface.routes

request = RequestFactory().get("/route/generate?lat=51.0&lon=3.7&min_length=45.0"
                               "&max_length=50.0&poison_max_value=120.0&poison_max_distance=10.0")
print(request.path)
start = time.time()
cProfile.runctx('server.interface.routes.route_from_coord(request)', None, locals(), filename='out.prof')
duration = time.time() - start
print("It took %s seconds." % duration)

import pstats
p = pstats.Stats('out.prof')
p.strip_dirs().sort_stats(1).print_stats()
