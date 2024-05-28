use egui::include_image;
use serde::{Deserialize, Serialize};

use super::ICON_SIZE;

pub mod home;
pub mod settings;

pub const ALL: [Tab; 2] = [Tab::Home, Tab::Settings];

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Tab {
    #[default]
    Home,
    Settings,
}

impl Tab {
    pub const fn title(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Settings => "Settings",
        }
    }

    /// Whether the tab is expandable.
    // FIXME: `tab` might not be the best name for this. It's not a tab, it's a
    // component of a panel. It's not a `Button` because it represents more than
    // that. It's a thingie that can be expanded or not.
    pub const fn expandable(self) -> bool {
        match self {
            Self::Home => false,
            Self::Settings => true,
        }
    }

    pub fn default_state(self) -> State {
        match self {
            Self::Home => State::Home(Default::default()),
            Self::Settings => State::Settings(Default::default()),
        }
    }

    pub fn ui(self, ui: &mut egui::Ui, state: &mut State) {
        state.ui(ui);
    }

    pub const fn icon(self) -> egui::ImageSource<'static> {
        match self {
            Self::Home => include_image!("../../assets/home.svg"),
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
    Home(home::Home),
    Settings(settings::Settings),
}

impl State {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::Home(_) => {}
            Self::Settings(settings) => settings.ui(ui),
        }
    }
}
