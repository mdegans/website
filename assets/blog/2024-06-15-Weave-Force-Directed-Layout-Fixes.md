Today I made a few more fixes to the force-directed layout. Previously, the time step was a constant, but now it's based on the actual frame time. Now It's possible to drag the window from a 120hz screen to a 60hz screen without affecting the simulation.

Debug view comes in handy for this. Currently drawn are the node rectangle (plus padding), the gravitational centroids, and the velocity of each node. A green centroid is the global, red local, and yellow, the weighted average. Velocity is represented as a red edge around each node. The direction is not shown, only the magnitude, but this will probably change in the future.

![Debug view](https://raw.githubusercontent.com/mdegans/website/main/assets/blog/force-directed-layout-debug.png)

A slider has also been added to control a speed multiplier. This is useful in situations where the simulation is too slow or too fast. The feature is almost bug-free. After that there are a few crashes and outstanding issues on the tracker before the next release but compared to last week this is a much better place to be in.
