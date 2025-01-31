use egui::Context;

use crate::app::{App, View};

pub struct LogoutView {}

impl LogoutView {
    pub fn build(ctx: &Context, app: &mut App) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Logout").clicked() {
                app.current_view = View::Login;
            }
        })
    }
}
