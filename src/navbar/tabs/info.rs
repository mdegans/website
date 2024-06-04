/// Home tab
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Info;

impl Info {
    pub fn ui(&mut self, _ui: &mut egui::Ui) {
        // nothing here yet
    }
}
