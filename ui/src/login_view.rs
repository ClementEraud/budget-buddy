use egui::Context;

use crate::app::{App, View};

pub struct LoginView {}

impl LoginView {
    pub fn build(ctx: &Context, app: &mut App) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Login").clicked() {
                app.current_view = View::Home;
            }
        })
    }
}
