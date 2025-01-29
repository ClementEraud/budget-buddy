use egui::Context;

use crate::{TemplateApp, View};

pub struct LogoutView {}

impl LogoutView {
    pub fn build(ctx: &Context, app: &mut TemplateApp) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Logout").clicked() {
                app.current_view = View::Login;
            }
        })
    }
}
