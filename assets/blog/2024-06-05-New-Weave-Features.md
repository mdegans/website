Today I added some more features to [Weave](https://github.com/mdegans/weave), bringing it closer to the functionality of the original [`loom`](https://github.com/socketteer/loom) project by [socketteer](https://github.com/socketteer). I'm happy with the progress so far and I'm looking forward to adding more features in the coming days.

Markdown support is now available for generated text as well as for help. Keyboard shortcuts have been added for common actions and some code has been reorganized to make it easier to add new features in the future.

The easiest thing to add next is probably a tabbed ui. There's a Rust crate for [`egui`] that does what I want. Having multiple tabs that can be torn off and moved around will be a nice addition to the app. "Combine multiple trees" and "Change tree topology" are also good candidates for the next update.

Harder to accomplish is scrolling and zooming for the main viewport, since [`egui`] does not natively feature this functionality. I'll probably build a new release at the end of the week since I've made a lot of changes since the last one. I'm looking forward to seeing what people create with it.

[`egui`]: https://github.com/emilk/egui
