use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings;

impl Settings {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Egui Settings")
            .show(ui, |ui| {
                ui.ctx().clone().settings_ui(ui);
            })
            .header_response
            .on_hover_text_at_pointer("Settings for the egui UI");
        egui::CollapsingHeader::new("Inspection")
            .show(ui, |ui| {
                ui.ctx().clone().inspection_ui(ui);
            })
            .header_response
            .on_hover_text_at_pointer("Tools to inspect the UI state.");
        egui::CollapsingHeader::new("Memory")
            .show(ui, |ui| {
                ui.ctx().clone().memory_ui(ui);
            })
            .header_response
            .on_hover_text_at_pointer("Memory debugging. Reset gui state, etc.");
    }
}
