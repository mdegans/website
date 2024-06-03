//! A [`Blog`] struct that displays all blog [`entries`].
use egui_infinite_scroll::InfiniteScroll;

pub mod entries;

/// The number of entries to display at once.
const N_ENTRIES: usize = 20;

#[derive(Default)]
pub struct Blog {
    scroll: InfiniteScroll<(), usize>,
    cache: egui_commonmark::CommonMarkCache,
}

impl Blog {
    pub fn new() -> Self {
        let scroll =
            InfiniteScroll::new().end_loader(move |cursor, callback| {
                let cursor = cursor.unwrap_or(0);
                // since our data is static, we store nothing in the actual
                // vec and just use the indices to reference the entries
                let items: Vec<_> = [(); entries::ENTRIES.len()]
                    .iter()
                    .skip(cursor)
                    .take(N_ENTRIES)
                    .cloned()
                    .collect();
                callback(Ok((items, Some(cursor + N_ENTRIES))));
            });

        Self {
            scroll,
            cache: Default::default(),
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
    ) -> egui::CollapsingResponse<egui::scroll_area::ScrollAreaOutput<()>> {
        egui::CollapsingHeader::new("Blog")
            .default_open(false)
            .show(ui, |ui| {
                let response = egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        egui_commonmark::commonmark_str!(
                            "Blog",
                            ui,
                            &mut self.cache,
                            "assets/blog.md"
                        );

                        self.scroll.ui(ui, N_ENTRIES, |ui, i, _| {
                            entries::ENTRIES[i].ui(ui, &mut self.cache);
                        });
                    });

                response
            })
    }
}
