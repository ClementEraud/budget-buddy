use egui::{FontId, Image, ImageSource, RichText, Sense, Ui, Vec2};

use crate::{app::App, views::View};

pub struct Avatar<'a> {
    img_src: ImageSource<'a>,
    name: String,
}

impl<'a> Avatar<'a> {
    pub fn new(img_src: ImageSource<'a>, name: String) -> Avatar<'a> {
        Avatar { img_src, name }
    }

    pub fn build(self, ui: &mut Ui, app: &mut App) {
        ui.vertical(|ui| {
            let btn = ui.add(
                Image::new(self.img_src)
                    .fit_to_exact_size(Vec2 { x: 150.0, y: 150.0 })
                    .sense(Sense::click()),
            );
            if btn.clicked() {
                app.navigate(View::Home);
            }

            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(RichText::new(self.name).font(FontId::proportional(20.0)));
            });
        });
    }
}
