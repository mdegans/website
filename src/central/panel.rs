use egui::include_image;
use egui_commonmark::CommonMarkCache;

use crate::navbar::{tabs::Tab, ICON_SIZE};

/// Central panel
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Panel {
    /// Commonmark cache
    #[serde(skip)]
    commonmark_cache: CommonMarkCache,
    /// Gallery of images
    #[serde(skip)]
    gallery: crate::gallery::Gallery,
}

impl Panel {
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        navbar: &mut crate::Navbar,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match navbar.selected_tab {
                Tab::Info | Tab::Settings => self.draw_home(ui, navbar),
            }
        });
    }

    pub fn draw_home(&mut self, ui: &mut egui::Ui, navbar: &mut crate::Navbar) {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.horizontal(|ui| {
            ui.heading("Michael de Gans");
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::RIGHT),
                |ui| {
                    let menu_btn = egui::Button::image(include_image!(
                        "../../assets/menu.svg"
                    ))
                    .frame(navbar.expanded);

                    if ui.add_sized([ICON_SIZE, ICON_SIZE], menu_btn).clicked()
                    {
                        navbar.expanded = !navbar.expanded;
                    }
                },
            );
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            crate::common::about(ui, &mut self.commonmark_cache);
            crate::common::projects(ui, &mut self.commonmark_cache);
            crate::common::hobbies(
                ui,
                &mut self.commonmark_cache,
                &mut self.gallery,
            );
        });
    }
}
