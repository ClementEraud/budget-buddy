use egui::Context;

use crate::app::App;

pub struct Home {}

impl Home {
    pub fn build(ctx: &Context, _app: &mut App) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Home");
        })
    }
}
