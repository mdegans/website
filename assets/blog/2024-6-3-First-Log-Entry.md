This is my first log entry. I'm excited to start this journey and see where it takes me. I'm not sure what to expect, but I'm looking forward to the adventure.

In addition to this website, recently I've been adding to my [collaborative writing app](https://github.com/mdegans/weave). It started off as a feature demo in the [`drama_llama`](https://github.com/mdegans/drama_llama) crate but quickly outgrew it's bounds.

I came up with the idea for [`Weave`](https://github.com/mdegans/weave) after seeing what a python developer had done with the OpenAI API and their [`loom`](https://github.com/socketteer/loom) project.

I wanted to build something similar, but in Rust, and with support for local models. This way the app is not dependent on any one provider and can be used offline.

Why Rust? I like the language and strong typing cuts down on bugs. I've been getting familiar with it for a while now and it's a good upgrade from Python, C, and C++.

It's also nice being able to build a small, fast, and efficient binary that can run on a variety of platforms. I've been able to build for macOS (Metal) and Linux (CUDA) so far.

Windows will come eventually, but I'm not in a rush. I personally don't use it for anything other than gaming and it's likely, but untested, that the Linux build will work under WSL.
