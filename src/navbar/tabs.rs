use egui::include_image;
use serde::{Deserialize, Serialize};

use super::ICON_SIZE;

pub mod info;
pub mod settings;

pub const ALL: [Tab; 2] = [Tab::Info, Tab::Settings];

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Tab {
    #[default]
    Info,
    Settings,
}

impl Tab {
    pub const fn title(self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Settings => "Settings",
        }
    }

    pub fn default_state(self) -> State {
        match self {
            Self::Info => State::Info(Default::default()),
            Self::Settings => State::Settings(Default::default()),
        }
    }

    pub const fn icon(self) -> egui::ImageSource<'static> {
        match self {
            Self::Info => include_image!("../../assets/info.svg"),
            Self::Settings => include_image!("../../assets/settings.svg"),
        }
    }

    pub fn button(self) -> egui::Button<'static> {
        let image: egui::widgets::Image<'_> = self.icon().into();
        egui::Button::image(image).rounding(ICON_SIZE / 3.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Info(info::Info),
    Settings(settings::Settings),
}

impl State {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::Info(info) => info.ui(ui),
            Self::Settings(settings) => settings.ui(ui),
        }
    }
}
