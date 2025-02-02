use container::Container;
use egui::{CentralPanel, Context, Grid, Vec2};

use crate::app::App;

mod container;

pub struct Home {}

impl Home {
    pub fn build(ctx: &Context, _app: &mut App) {
        CentralPanel::default().show(ctx, |ui| {
            Grid::new("home_grid")
                .min_col_width(ui.available_width())
                .min_row_height(ui.available_height() / 2.0)
                .spacing(Vec2 { x: 0.0, y: 20.0 })
                .show(ui, |ui| {
                    Container::new(String::from("Planned")).build(ui);
                    ui.end_row();

                    Container::new(String::from("Actual")).build(ui);
                    ui.end_row();
                });
        });
    }
}
