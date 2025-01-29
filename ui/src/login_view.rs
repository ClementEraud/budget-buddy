use egui::Context;

use crate::{TemplateApp, View};

pub struct LoginView {}

impl LoginView {
    pub fn build(ctx: &Context, app: &mut TemplateApp) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Login").clicked() {
                app.current_view = View::Home;
            }
        })
    }
}
