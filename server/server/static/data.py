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
print "Loaded the initial graph..."

# A projector that maps crossroads on a plane.
# Type : Projector
PROJECTOR = project_city([GRAPH.get(ident) for ident in GRAPH.list_ids()])
print "Loaded the projector..."

# Annotate the graph with xy data
GRAPH = city.project(GRAPH, PROJECTOR)
print "Mapped x/y projections to all nodes..."

# The interval containing every projected road and crossroad of the city.
# Type : Interval<()>
CITY_BOUNDS = reduce(lambda x, y: x + y, 
    (into_interval(node, node, 0.0) for node in GRAPH.iter_nodes()))
print "Computed city bounds..."

# A grid containing GRAPH, for faster access.
GRID = CITY_BOUNDS.into_grid(BINSIZE)
print "Created the city grid..."

for startnode in GRAPH.list_ids():
    for endnode in GRAPH.get_connids(startnode):
        GRID.add_interval(into_interval(GRAPH.get(startnode), GRAPH.get(endnode), TOLERANCE))

print "Mapped all edges to the grid..."

end = time.time()
print("TOTAL GRAPH TIME", end - start)

# Debug stuff. Can be left out.
#store_coverage(GRID)
#store_graph(GRAPH)
print("Ready")