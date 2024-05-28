use egui::{include_image, TextureOptions};
use serde::{Deserialize, Serialize};

use super::ICON_SIZE;

pub mod settings;

pub const ALL: [Tab; 1] = [Tab::Settings];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tab {
    #[default]
    Settings,
}

impl Tab {
    pub const fn title(&self) -> &str {
        match self {
            Self::Settings => "Settings",
        }
    }

    pub fn default_state(&self) -> State {
        match self {
            Self::Settings => State::Settings(Default::default()),
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui, state: &mut State) {
        state.ui(ui);
    }

    pub const fn icon(&self) -> egui::ImageSource<'_> {
        match self {
            Self::Settings => include_image!("../../assets/settings.svg"),
        }
    }

    pub fn button(&self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_sized(
            egui::Vec2::new(ICON_SIZE, ICON_SIZE),
            egui::Button::image(self.icon()).rounding(ICON_SIZE / 3.0),
        )
        .on_hover_text_at_pointer(self.title())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Settings(settings::Settings),
}

impl State {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::Settings(settings) => settings.ui(ui),
        }
    }
}
