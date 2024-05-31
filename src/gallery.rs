use std::{fmt::format, path::PathBuf};

use egui::Vec2;
// This is adapted from the egui_infinite_scroll example
use egui_infinite_scroll::InfiniteScroll;
use serde::Deserialize;

#[cfg_attr(test, derive(serde::Serialize))] // A test generates the JSON
#[derive(Debug, Clone, Deserialize)]
struct Item {
    filename: PathBuf,
    width: usize,
    height: usize,
}

#[derive(Default)]
pub struct Gallery {
    scroll: InfiniteScroll<Item, usize>,
}

impl Gallery {
    pub fn new(json_str: &str) -> Self {
        let data: Vec<Item> = serde_json::from_str(json_str).unwrap();
        print!("{}", data.len());
        let scroll =
            InfiniteScroll::new().end_loader(move |cursor, callback| {
                let cursor = cursor.unwrap_or(0);
                let items: Vec<_> =
                    data.iter().skip(cursor).take(20).cloned().collect();
                callback(Ok((items, Some(cursor + 20))));
            });

        Self { scroll }
    }
}

impl Gallery {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
    ) -> egui::scroll_area::ScrollAreaOutput<()> {
        const MAX_COL_WIDTH: usize = 256;
        const SPACING: f32 = 16.0;

        let response = egui::ScrollArea::vertical()
            .max_height(ui.available_height() - SPACING * 2.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(SPACING);
                let n_columns = (ui.available_width() / MAX_COL_WIDTH as f32).ceil() as usize;
                self.scroll.ui_columns(
                    ui,
                    10,
                    n_columns,
                    None,
                    |ui, _, item| {
                        let width = ui.available_width();
                        let height = width * (item.height as f32 / item.width as f32);
                        let path = format!(
                            "https://raw.githubusercontent.com/mdegans/website/main/{}",
                            item.filename.to_string_lossy()
                        );

                        ui.add_sized([width, height], egui::Image::new(&path));
                    },
                )
            });

        ui.add_space(SPACING);

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Image dimensions helper.
    mod helper {
        use std::path::Path;

        pub fn image_dimensions(path: &Path) -> (usize, usize) {
            let image = image::io::Reader::open(path).unwrap();
            let image = image.decode().unwrap();
            (image.width() as usize, image.height() as usize)
        }
    }

    /// Generate gallery json from a path
    fn gen_gallery_json(json_path: &str, gallery_path: &str) {
        const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
        let in_ = Path::new(CRATE_ROOT).join(gallery_path);
        let out = Path::new(CRATE_ROOT).join(json_path);

        let items: Vec<Item> = in_
            .read_dir()
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().extension().map_or(false, |ext| ext == "jpg")
            })
            .map(|entry| {
                let abspath = entry.path();
                let filename = abspath.file_name().unwrap();

                let (width, height) = helper::image_dimensions(&abspath);
                let mut root_relpath = PathBuf::from(gallery_path);
                root_relpath.push(filename);

                Item {
                    filename: root_relpath,
                    width,
                    height,
                }
            })
            .collect();

        let json = serde_json::to_string_pretty(&items).unwrap();
        std::fs::write(out, json).unwrap();
    }

    /// This test is just to generate the json for all .jpg files in the
    /// assets/gallery folder. This should be run whenever new images are added.
    #[test]
    #[ignore = "reason: test generates JSON"]
    fn gallery_json() {
        const JSON_PATH: &str = "assets/gallery.json";
        const GALLERY_PATH: &str = "assets/gallery";
        gen_gallery_json(JSON_PATH, GALLERY_PATH);
    }

    /// This does the same as above, only for the dog photo gallery.
    #[test]
    fn doggos_json() {
        const JSON_PATH: &str = "assets/doggos.json";
        const GALLERY_PATH: &str = "assets/doggos";
        gen_gallery_json(JSON_PATH, GALLERY_PATH)
    }
}
