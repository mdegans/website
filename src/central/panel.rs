use std::any::Any;

use egui_commonmark::CommonMarkCache;

use crate::navbar::{self, tabs::Tab};

/// Central panel
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Panel {
    /// Commonmark cache
    #[serde(skip)]
    commonmark_cache: CommonMarkCache,
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
                Tab::Home | Tab::Settings => self.draw_home(ui),
            }
        });
    }

    pub fn draw_home(&mut self, ui: &mut egui::Ui) {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("Michael de Gans");

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            crate::common::projects(ui, &mut self.commonmark_cache);
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.add(egui::github_link_file!(
                "https://github.com/mdegans/website/",
                "Website source code."
            ));
            ui.separator();
        });
    }
}
