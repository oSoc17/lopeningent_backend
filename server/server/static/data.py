from server.logic.projection.util import project_city
from server.logic.server_util import time_fn
from server.logic.grid.interval import into_interval
from server.logic.graph.debug import store_coverage, store_graph
from server.config import TOLERANCE, BINSIZE
from server.logic.graph.graph import Graph
from server.logic.city import city
import time

start = time.time()

# A graph containing the roads and crossroads of the city.
# Type: Graph
GRAPH = city.load()

# A projector that maps crossroads on a plane.
# Type : Projector
PROJECTOR = project_city([GRAPH.get(ident) for ident in GRAPH.list_ids()])

# Annotate the graph with xy data
GRAPH = city.project(GRAPH, PROJECTOR)

# The interval containing every projected road and crossroad of the city.
# Type : Interval<()>
CITY_BOUNDS = reduce(lambda x, y: x + y, 
    (into_interval(node, node, 0.0) for node in GRAPH.iter_nodes()))

# A grid containing GRAPH, for faster access.
GRID = CITY_BOUNDS.into_grid(BINSIZE)

for startnode in GRAPH.list_ids():
    for endnode in GRAPH.get_connids(startnode):
        GRID.add_interval(into_interval(GRAPH.get(startnode), GRAPH.get(endnode), TOLERANCE))

end = time.time()
print "Graph loaded into memory (took {} seconds).".format(round(end - start, 2))