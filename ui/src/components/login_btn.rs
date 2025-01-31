use egui::Ui;

use crate::{app::App, views::View};

pub struct LoginBtn {}

impl LoginBtn {
    pub fn build(ui: &mut Ui, app: &mut App) {
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
        );
        if ui.button("Log in").clicked() {
            app.navigate(View::Home);
        }
    }
}
