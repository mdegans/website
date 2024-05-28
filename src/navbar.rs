use std::collections::HashMap;

use egui::{include_image, panel::Side};
use serde::{Deserialize, Serialize};

pub mod tabs;

pub const ICON_SIZE: f32 = 30.0;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Navbar {
    /// If the navbar is expanded.
    pub expanded: bool,
    /// If None, the [`Navbar`] is hidden, but we maintain the state of the tabs.
    pub selected_tab: tabs::Tab,
    /// A mapping of tabs to their respective state.
    pub state: HashMap<tabs::Tab, tabs::State>,
}

impl Navbar {
    pub fn show(&mut self, ctx: &egui::Context) {
        // If we're expanded, we show a side panel with a tab bar.
        egui::SidePanel::new(Side::Left, "Navbar").show_animated(ctx, self.expanded, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        egui::Vec2::new(ICON_SIZE, ICON_SIZE),
                        egui::Button::image(include_image!("../assets/back.svg"))
                            .rounding(ICON_SIZE / 3.0)
                            .frame(true),
                    )
                    .on_hover_text_at_pointer("Back")
                    .clicked()
                {
                    self.expanded = false;
                }

                ui.heading(self.selected_tab.title())
            });

            let state = self
                .state
                .entry(self.selected_tab)
                .or_insert(self.selected_tab.default_state());

            state.ui(ui);
        });
        // If we're collapsed, we just show a vertical button bar.
        egui::SidePanel::new(Side::Left, "ButtonBar")
            .exact_width(30.0)
            .resizable(false) // exact_width does not imply this so the mouse
            // cursor is wrong without it
            .show_animated(ctx, !self.expanded, |ui| {
                ui.vertical(|ui| {
                    for tab in tabs::ALL {
                        if tab.button(ui).clicked() {
                            self.selected_tab = tab;
                            self.expanded = true;
                        }
                    }
                })
            });
    }
}
