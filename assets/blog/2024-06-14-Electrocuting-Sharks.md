Yesterday I finished up the force-directed layout for Weave. I've been working on it for a few days now and I'm happy with the results. I've tweaked the parameters to the point where the layout is stable and looks good. There are
still failure cases, but fewer of them. I'll address them in a separate commit.

Today I fixed a model loading issue. I hadn't extensively tested the model
loading code and it turned out that it would't load until the app was restarted.
I fixed that now and changed loading to be asynchronous. Loading now happens
in a separate thread and the app is usable while it's happening.

As with the OpenAI backend, I'm using channels to communicate between the
main thread and worker thread. This is a pattern I've used before and it
works well. I send `Request` objects to the worker thread and it sends
`Response` objects back. This way I can keep the main ui thread responsive
no matter what.

To test everything out, I created a few text nodes from the former president's
recent speech on electrocuting sharks. I was curious to see how well the model
could predict divergent paths from the ground truth text. Not surprisingly,
the generated text is nearly indistinguishable from the real thing. Not only
that, because I framed the text as a WaPo article, the model even generated
commentary pointing out the logical inconsistencies in the speech.

A screenshot of that is
[here](https://raw.githubusercontent.com/mdegans/website/main/blog/assets/sharks.png)
and the JSON to load it into Weave is
[here](https://raw.githubusercontent.com/mdegans/website/main/blog/assets/sharks.json).
Note that the json was generated with the dev branch of Weave and may not work
with the current release.
