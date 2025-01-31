use egui::{epaint::CircleShape, Color32, Context, Rect, Stroke, Vec2};

use crate::app::{App, View};

pub struct LoginView {}

impl LoginView {
    pub fn build(ctx: &Context, app: &mut App) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a circle
            let available_rect = ui.available_rect_before_wrap();
            let image_pos = egui::pos2(
                available_rect.min.x + available_rect.width() / 2.0,
                available_rect.min.y + available_rect.height() / 3.0,
            );
            let radius = 80.0;
            let circle = CircleShape {
                center: image_pos,
                radius,
                stroke: Stroke::NONE,
                fill: Color32::LIGHT_GRAY,
            };

            // Add the circle to the UI
            let painter = ui.painter();
            painter.circle(circle.center, circle.radius, circle.fill, circle.stroke);

            // Load the image
            let image_size = Vec2 { x: 150.0, y: 100.0 };
            let rect = Rect::from_center_size(image_pos, image_size);
            egui::Image::new(egui::include_image!("../../assets/ferris.png"))
                .tint(Color32::LIGHT_GRAY)
                .paint_at(ui, rect);

            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0);
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
                );
                if ui.button("Log in").clicked() {
                    app.navigate(View::Home);
                }
            });
        })
    }
}
