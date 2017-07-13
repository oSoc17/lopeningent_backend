import database
from logic.graph.graph import Graph

if __name__ == "__main__":
    edgelist, nodelist = database.get_graph_data()
    print len(edgelist), len(nodelist)
    