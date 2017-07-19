from server.logic.grid.interval import into_interval

###############################################################################
# DEBUGGING AND PRINTING STATIC DATA                                          #
###############################################################################


def store_coverage(grid):
    """ output filled cells to text file "Ghent.txt" """
    with open("ghent.txt", "w+") as f:
        for row in grid.data:
            f.write("%s\n" % ''.join("  " if len(field)
                                     == 0 else "##" for field in row))
        print("SIZE: %i %i" % (len(grid.data), len(grid.data[0])))


def store_graph(graph):
    """ output city roads to svg file. """
    bounds = reduce(lambda x, y: x + y, (into_interval(node, node, 0.0)
                                         for node in graph.iter_nodes()))

    SCALE = 100
    
    with open("ghent.svg", "w+") as f:
        f.write('<svg xmlns="http://www.w3.org/2000/svg" \
            xmlns:xlink="http://www.w3.org/1999/xlink">\n')
        for edge in graph.iter_edges():
            f.write('<line x1="%f" y1="%f" x2="%f" y2="%f" style="stroke:#000000;"/>\n' %
                    ((-graph.get(edge.id).x + bounds.maxx) * SCALE,
                     (-graph.get(edge.id).y + bounds.maxy) * SCALE,
                     (-graph.get(edge.to).x + bounds.maxx) * SCALE,
                     (-graph.get(edge.to).y + bounds.maxy) * SCALE))
        f.write("</svg>")

    with open("edges.txt", "w+") as f:
        for edge in graph.iter_edges():
            f.write("{}".format(edge))
            f.write("\n")
