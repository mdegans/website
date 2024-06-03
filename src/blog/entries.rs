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

pub const ENTRIES: &[&Entry] = &[&Entry::new(
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
)];
