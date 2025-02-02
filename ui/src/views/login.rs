use egui::{include_image, Context, FontId, RichText};

use crate::{app::App, components::avatar::Avatar};

pub struct Login {}

impl Login {
    pub fn build(ctx: &Context, app: &mut App) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(ui.available_height() / 5.0);

            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() / 3.0);

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Who is using Budget Buddy ?")
                            .font(FontId::proportional(50.0)),
                    );

                    ui.add_space(50.0);

                    ui.horizontal(|ui| {
                        ui.add_space(250.0);

                        Avatar::new(
                            include_image!("../../../assets/ferris.png"),
                            String::from("Default User"),
                        )
                        .build(ui, app);
                    });
                })
            });
        });
    }
}
