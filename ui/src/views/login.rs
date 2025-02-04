use avatar::Avatar;
use egui::{include_image, FontId, RichText, Ui};

use crate::app::App;

mod avatar;

pub struct Login {}

impl Login {
    pub fn build(ui: &mut Ui, app: &mut App) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.label(RichText::new("Who is using Budget Buddy ?").font(FontId::proportional(50.0)));

            ui.add_space(50.0);

            Avatar::new(
                include_image!("../../../assets/ferris.png"),
                String::from("Default User"),
            )
            .build(ui, app);
        });
    }
}
