use egui::Context;

use crate::{
    app::App,
    components::{avatar::Avatar, login_btn::LoginBtn},
};

pub struct Login {}

impl Login {
    pub fn build(ctx: &Context, app: &mut App) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            let avatar = Avatar::new(egui::include_image!("../../../assets/ferris.png"));
            avatar.build(ui);

            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.3);
                LoginBtn::build(ui, app);
            });
        })
    }
}
