//! Blog entries.
use time::{Date, Month};

// A map of numeric months to their string representations.
const MONTHS: &[Month] = &[
    Month::January, // 0
    Month::January,
    Month::February,
    Month::March,
    Month::April,
    Month::May,
    Month::June,
    Month::July,
    Month::August,
    Month::September,
    Month::October,
    Month::November,
    Month::December,
];

/// A blog entry.
///     
/// # Fields
/// - `date`: The date the entry was published.
/// - `title`: The title of the entry.
/// - `content`: The content of the entry
#[derive(Debug)]
pub struct Entry {
    pub date: Date,
    pub title: &'static str,
    pub content: &'static str,
}

impl Entry {
    /// Create a new blog entry.
    pub const fn new(
        year: i32,
        month: u8,
        date: u8,
        title: &'static str,
        content: &'static str,
    ) -> Self {
        Self {
            date: match Date::from_calendar_date(
                year,
                MONTHS[month as usize],
                date,
            ) {
                Ok(date) => date,
                Err(_) => panic!("Invalid date"),
            },
            title,
            content,
        }
    }

    /// Entry UI.
    pub fn ui(
        &self,
        ui: &mut egui::Ui,
        commonmark_cache: &mut egui_commonmark::CommonMarkCache,
    ) -> egui::CollapsingResponse<egui::InnerResponse<()>> {
        let title = format!("{} - {}", self.date, self.title);

        egui::CollapsingHeader::new(title)
            .default_open(false)
            .show(ui, |ui| {
                egui_commonmark::CommonMarkViewer::new(self.title).show(
                    ui,
                    commonmark_cache,
                    self.content,
                )
            })
    }
}

pub const ENTRIES: &[&Entry] = &[
    &Entry::new(
        2024,
        6,
        14,
        r#"Electrocuting Sharks"#,
        r#"Yesterday I finished up the force-directed layout for Weave. I've been working on it for a few days now and I'm happy with the results. I've tweaked the parameters to the point where the layout is stable and looks good. There are
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
"#,
    ),
    &Entry::new(
        2024,
        6,
        12,
        r#"Weave UI Enhancements"#,
        r#"Today I finished up the force-directed layout for Weave. I've been working on it for a few days now and I'm happy with the results. I've tweaked the parameters to the point where the layout is stable and looks good. There are failure cases, such as when all nodes overlap, but they'll be addressed in a separate commit.

Additionally, I polished up the UI a bit by adding icons as well as adding an in-app trash can for deleted stories. This way it takes more than a single click to delete a story which might have taken a lot of work to create.

Tomorrow, in addition to some resumé updates, I'll be addressing some of the bugs I've found over the past few days as well as the failure cases in the force-directed layout. Likely I'll add random noise to positions that are too close together, or provide an option to randomize the node positions before layout.

Also of issue is the fact that there are some cases where high velocity nodes can end up outside the viewport. This shouldn't happen but it does. I know why but I haven't yet decided on the best way to address it. There is some inconsistency in the way node positions are handled that I need to address.
"#,
    ),
    &Entry::new(
        2024,
        6,
        11,
        r#"Weave Force Directed Layout"#,
        r#"Over the past few days I've been adding force-directed layout to Weave. I've had some experience with n-body simulations, but hadn't done anything for graphs. I ended up looking at how Gephi had done it.

The basic idea is to treat each node as a charged particle and each edge as a spring. The nodes repel each other and the edges pull them together. Normally this is quadratic in complexity, but I chose to only calculate the forces between nodes that are connected by an edge as well as between immediate children. A similar optimization is used by Gephi. In cases where there is one parent and many children, this can still be quadratic, but in the general case it's closer to linear, and fast enough for node layout.

There is also approximate gravity which pulls nodes towards both a local centroid and a global one. This helps to keep the graph from flying apart. The local centroid is the average position of the immediate children of a node and the global one is the average position of all nodes. I'm still tinkering with the parameters, but it's looking good so far. Children tend to clump and the tree doesn't fly apart. Colissions are still an issue, but that'll be among the next things I address.

Probably, a naive, quadratic implementation would have been fine but I want to allow for very large trees and this should scale to many thousands of nodes. At that point the drawing is likely to be the bottleneck, but I'll cross that bridge when I come to it. A scrollable, zoomable viewport is also on the list of things to add.

I've found quite a few bugs in the process, which I'll have to address before I can release the next version. I'll start on that once I've found good default parameters for the layout.
"#,
    ),
    &Entry::new(
        2024,
        6,
        5,
        r#"New Weave Features"#,
        r#"Today I added some more features to [Weave](https://github.com/mdegans/weave), bringing it closer to the functionality of the original [`loom`](https://github.com/socketteer/loom) project by [socketteer](https://github.com/socketteer). I'm happy with the progress so far and I'm looking forward to adding more features in the coming days.

Markdown support is now available for generated text as well as for help. Keyboard shortcuts have been added for common actions and some code has been reorganized to make it easier to add new features in the future.

The easiest thing to add next is probably a tabbed ui. There's a Rust crate for [`egui`] that does what I want. Having multiple tabs that can be torn off and moved around will be a nice addition to the app. "Combine multiple trees" and "Change tree topology" are also good candidates for the next update.

Harder to accomplish is scrolling and zooming for the main viewport, since [`egui`] does not natively feature this functionality. I'll probably build a new release at the end of the week since I've made a lot of changes since the last one. I'm looking forward to seeing what people create with it.

[`egui`]: https://github.com/emilk/egui
"#,
    ),
    &Entry::new(
        2024,
        6,
        4,
        r#"New Website"#,
        r#"So I decided to write a new website, this time in Rust. Why? Because I'm tired of updating wordpress and getting spam comments for a comment section that is supposed to be both disabled and hidden. All for content that might as well be static.

I'm tired of the security issues. So, here we are with a static site because there isn't a reason not to. I write in markdown, which on save triggers `build.rs` which adds and entry in `/src/blog/entries.rs` which is built into the binary, and because text is highly compressible, the binary is small and fast, even storing all the blog entries in memory. All I need to do from there is commit and `git push` and the site is updated.

The markdown is parsed with [`egui-commonmark`](https://docs.rs/egui_commonmark/latest/egui_commonmark/) which creates appropriate widgets. There's also some [infinite scroll](https://docs.rs/egui_infinite_scroll/latest/egui_infinite_scroll/) but there's a small [issue](https://github.com/lucasmerlin/hello_egui/issues/26) where the scroll elements disappear when the window is refreshed. I'll track that down in time.

Eventually I'll grab the blog entries from elsewhere, like static files, but there isn't a reason to do that yet. So far so good. The site is fast and does what I want. I don't really care if scrapers can get to it since they'll just use it to train AI models. It's for human eyes. I could generate html from the markdown as well for scrapers, but I don't care enough to do that yet.
"#,
    ),
    &Entry::new(
        2024,
        6,
        3,
        r#"First Log Entry"#,
        r#"This is my first log entry. I'm excited to start this journey and see where it takes me. I'm not sure what to expect, but I'm looking forward to the adventure.

In addition to this website, recently I've been adding to my [collaborative writing app](https://github.com/mdegans/weave). It started off as a feature demo in the [`drama_llama`](https://github.com/mdegans/drama_llama) crate but quickly outgrew it's bounds.

I came up with the idea for [`Weave`](https://github.com/mdegans/weave) after seeing what a python developer had done with the OpenAI API and their [`loom`](https://github.com/socketteer/loom) project.

I wanted to build something similar, but in Rust, and with support for local models. This way the app is not dependent on any one provider and can be used offline.

Why Rust? I like the language and strong typing cuts down on bugs. I've been getting familiar with it for a while now and it's a good upgrade from Python, C, and C++.

It's also nice being able to build a small, fast, and efficient binary that can run on a variety of platforms. I've been able to build for macOS (Metal) and Linux (CUDA) so far.

Windows will come eventually, but I'm not in a rush. I personally don't use it for anything other than gaming and it's likely, but untested, that the Linux build will work under WSL.
"#,
    ),
];
