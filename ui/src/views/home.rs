use container::Container;
use egui::{Grid, Ui, Vec2};

use crate::app::App;

mod container;

pub struct Home {}

impl Home {
    pub fn build(ui: &mut Ui, _app: &mut App) {
        Grid::new("home_grid")
            .min_col_width(ui.available_width())
            .min_row_height(ui.available_height() / 2.0)
            .spacing(Vec2 { x: 0.0, y: 10.0 })
            .show(ui, |ui| {
                Container::new(String::from("Planned")).build(ui);
                ui.end_row();

                Container::new(String::from("Actual")).build(ui);
                ui.end_row();
            });
    }
}
