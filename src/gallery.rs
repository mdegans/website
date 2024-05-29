use std::path::PathBuf;

use egui::Vec2;
// This is adapted from the egui_infinite_scroll example
use egui_infinite_scroll::InfiniteScroll;
use serde::Deserialize;

#[cfg_attr(test, derive(serde::Serialize))] // A test generates the JSON
#[derive(Debug, Clone, Deserialize)]
struct Item {
    id: usize,
    title: String,
    filename: PathBuf,
    width: usize,
    height: usize,
}

pub struct Gallery {
    scroll: InfiniteScroll<Item, usize>,
}

impl Default for Gallery {
    fn default() -> Self {
        let data: Vec<Item> =
            serde_json::from_str(include_str!("../assets/gallery.json"))
                .unwrap();
        let scroll =
            InfiniteScroll::new().end_loader(move |cursor, callback| {
                let cursor = cursor.unwrap_or(0);
                let items: Vec<_> =
                    data.iter().skip(cursor).take(10).cloned().collect();
                if !items.is_empty() {
                    callback(Ok((items, Some(cursor + 10))));
                }
            });

        Self { scroll }
    }
}

impl Gallery {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
    ) -> egui::scroll_area::ScrollAreaOutput<()> {
        let height = 300.0;

        egui::ScrollArea::vertical()
            .max_height(ui.available_height() * 0.9)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(16.0);
                let item_spacing = ui.spacing().item_spacing.x;
                self.scroll.ui_custom_layout(ui, 10, |ui, start_idx, item| {
                    let total_width = ui.available_width();

                    let mut count = 1;
                    let mut combined_width = item
                        .first()
                        .map(|item| item.width as f32)
                        .unwrap_or(0.0);

                    while combined_width
                        < total_width - item_spacing * (count - 1) as f32
                        && count < item.len()
                    {
                        count += 1;
                        let item = &item[count - 1];
                        let item_aspect_ratio =
                            item.width as f32 / item.height as f32;
                        let item_width = height * item_aspect_ratio;
                        combined_width += item_width;
                    }

                    ui.horizontal(|ui| {
                        for (idx, item) in item.iter().enumerate().take(count) {
                            let path = format!(
                                "https://raw.githubusercontent.com/mdegans/website/main/assets/gallery/{}",
                                item.filename.to_string_lossy()
                            );
                            let response = ui
                                .add(egui::Image::new(path))
                                .on_hover_text(&item.title);
                        }
                    });

                    count
                });
            })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// Image dimensions helper.
    mod helper {
        use std::path::Path;

        pub fn image_dimensions(path: &Path) -> (usize, usize) {
            let image = image::io::Reader::open(path).unwrap();
            let image = image.decode().unwrap();
            (image.width() as usize, image.height() as usize)
        }
    }

    /// This test is just to generate the json for all .jpg files in the
    /// assets/gallery folder. This should be run whenever new images are added.
    #[test]
    #[ignore = "reason: test generates JSON"]
    fn gallery_json() {
        const JSON_PATH: &str = "assets/gallery.json";
        const GALLERY_PATH: &str = "assets/gallery";
        const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

        let in_ = Path::new(CRATE_ROOT).join(GALLERY_PATH);
        let out = Path::new(CRATE_ROOT).join(JSON_PATH);

        let items: Vec<Item> = in_
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "jpg")
            })
            .enumerate()
            .map(|(id, entry)| {
                let abspath = entry.path();
                let filename = abspath.file_name().unwrap();

                let (width, height) = helper::image_dimensions(&abspath);
                Item {
                    id,
                    title: filename.to_string_lossy().to_string(),
                    filename: filename.into(),
                    width,
                    height,
                }
            })
            .collect();

        let json = serde_json::to_string_pretty(&items).unwrap();
        std::fs::write(out, json).unwrap();
    }
}
