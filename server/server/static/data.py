from server.logic.projection.util import project_city
from server.logic.server_util import time_fn
from server.logic.grid.interval import into_interval
from server.logic.graph.debug import store_coverage, store_graph
from server.config import TOLERANCE, BINSIZE
from server.logic.graph.graph import Graph
from server.logic.city import city
import time, logging

start = time.time()

# Initialize logging
logging.basicConfig(filename="server.log", level=logging.DEBUG)

# A graph containing the roads and crossroads of the city.
# Type: Graph
logging.info("loading the graph structure into memory")
GRAPH = city.load()

DATABASE_EDGES = city.get_edges()

# A projector that maps crossroads on a plane.
# Type : Projector
logging.info("setting up the x/y projector")
PROJECTOR = project_city([GRAPH.get(ident) for ident in GRAPH.list_ids()])

# Annotate the graph with x/y data
logging.info("projecting the graph's nodes onto a x/y plane")
city.project(GRAPH, PROJECTOR)
logging.debug("graph nodes: %s", [GRAPH.get(ident) for ident in GRAPH.list_ids()])

# The interval containing every projected road and crossroad of the city.
# Type : Interval<()>
logging.info("calculating the city bounds")
CITY_BOUNDS = reduce(lambda x, y: x + y, 
    (into_interval(node, node, 0.0) for node in GRAPH.iter_nodes()))

# A grid containing GRAPH, for faster access.
logging.info("initializing a grid datstructure")
GRID = CITY_BOUNDS.into_grid(BINSIZE)

logging.info("putting all nodes into the grid datastructure")
for startnode in GRAPH.list_ids():
    for endnode in GRAPH.get_connids(startnode):
        GRID.add_interval(into_interval(GRAPH.get(startnode), GRAPH.get(endnode), TOLERANCE))

end = time.time()
print "Graph loaded into memory (took {} seconds).".format(round(end - start, 2))