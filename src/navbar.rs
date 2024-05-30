use std::collections::HashMap;

use egui::panel::Side;
use serde::{Deserialize, Serialize};

pub mod tabs;

pub const ICON_SIZE: f32 = 30.0;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Navbar {
    /// If the navbar is expanded.
    pub expanded: bool,
    /// The currently selected tab.
    pub selected_tab: tabs::Tab,
    /// A mapping of tabs to their respective state.
    pub state: HashMap<tabs::Tab, tabs::State>,
}

impl Navbar {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // If we're expanded, we show a side panel with the selected tab.
        egui::SidePanel::new(Side::Right, "Navbar")
            .resizable(false)
            .show_animated(ctx, self.expanded, |ui| {
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    for tab in tabs::ALL {
                        if ui
                            .add_sized(
                                [ICON_SIZE, ICON_SIZE],
                                tab.button().frame(tab == self.selected_tab),
                            )
                            .clicked()
                        {
                            self.selected_tab = tab;
                        }
                    }
                });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(self.selected_tab.title());

                    let state = self
                        .state
                        .entry(self.selected_tab)
                        .or_insert(self.selected_tab.default_state());

                    state.ui(ui);
                });
            });
    }
}
