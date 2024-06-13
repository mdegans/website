Today I finished up the force-directed layout for Weave. I've been working on it for a few days now and I'm happy with the results. I've tweaked the parameters to the point where the layout is stable and looks good. There are failure cases, such as when all nodes overlap, but they'll be addressed in a separate commit.

Additionally, I polished up the UI a bit by adding icons as well as adding an in-app trash can for deleted stories. This way it takes more than a single click to delete a story which might have taken a lot of work to create.

Tomorrow, in addition to some resumé updates, I'll be addressing some of the bugs I've found over the past few days as well as the failure cases in the force-directed layout. Likely I'll add random noise to positions that are too close together, or provide an option to randomize the node positions before layout.

Also of issue is the fact that there are some cases where high velocity nodes can end up outside the viewport. This shouldn't happen but it does. I know why but I haven't yet decided on the best way to address it. There is some inconsistency in the way node positions are handled that I need to address.
