use egui::{Color32, Ui};

use super::cell::Cell;

pub struct Incomes {}

impl Incomes {
    pub fn build(ui: &mut Ui) {
        Cell::new(String::from("Incomes"), Color32::LIGHT_GREEN).build(ui, |ui| {
            ui.label("text");
        });
    }
}
