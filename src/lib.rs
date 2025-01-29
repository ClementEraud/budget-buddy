#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

pub mod components {
    use crate::{app::View, TemplateApp};
    use egui::Context;

    pub fn top_bar(ctx: &Context) -> egui::InnerResponse<()> {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                })
            });
        })
    }

    pub fn login_view(ctx: &Context, app: &mut TemplateApp) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Login").clicked() {
                app.current_view = View::Home;
            }
        })
    }

    pub fn home_view(ctx: &Context, app: &mut TemplateApp) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Logout").clicked() {
                app.current_view = View::Login;
            }
        })
    }
}
