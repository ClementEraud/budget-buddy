use egui::{Color32, Ui};

use super::cell::Cell;

pub struct Balance {}

impl Balance {
    pub fn build(ui: &mut Ui) {
        Cell::new(String::from("Balance"), Color32::LIGHT_YELLOW).build(ui, |ui| {
            ui.label("text");
        });
    }
}
