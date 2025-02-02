use egui::{Color32, Ui};

use super::cell::Cell;

pub struct Expenses {}

impl Expenses {
    pub fn build(ui: &mut Ui) {
        Cell::new(String::from("Expenses"), Color32::LIGHT_RED).build(ui, |ui| {
            ui.label("text");
        });
    }
}
