use crate::app::{App, View};
use egui::Context;

pub struct TopBar {}

impl TopBar {
    pub fn build(ctx: &Context, app: &mut App) -> egui::InnerResponse<()> {
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

                    if app.current_view() != &View::Login {
                        if ui.button("Logout").clicked() {
                            app.navigate(View::Login);
                        }
                    }
                })
            });
        })
    }
}
