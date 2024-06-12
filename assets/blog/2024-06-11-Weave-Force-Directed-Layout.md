Over the past few days I've been adding force-directed layout to Weave. I've had some experience with n-body simulations, but hadn't done anything for graphs. I ended up looking at how Gephi had done it.

The basic idea is to treat each node as a charged particle and each edge as a spring. The nodes repel each other and the edges pull them together. Normally this is quadratic in complexity, but I chose to only calculate the forces between nodes that are connected by an edge as well as between immediate children. A similar optimization is used by Gephi. In cases where there is one parent and many children, this can still be quadratic, but in the general case it's closer to linear, and fast enough for node layout.

There is also approximate gravity which pulls nodes towards both a local centroid and a global one. This helps to keep the graph from flying apart. The local centroid is the average position of the immediate children of a node and the global one is the average position of all nodes. I'm still tinkering with the parameters, but it's looking good so far. Children tend to clump and the tree doesn't fly apart. Colissions are still an issue, but that'll be among the next things I address.

Probably, a naive, quadratic implementation would have been fine but I want to allow for very large trees and this should scale to many thousands of nodes. At that point the drawing is likely to be the bottleneck, but I'll cross that bridge when I come to it. A scrollable, zoomable viewport is also on the list of things to add.

I've found quite a few bugs in the process, which I'll have to address before I can release the next version. I'll start on that once I've found good default parameters for the layout.
