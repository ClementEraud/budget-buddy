use egui::{Align, Color32, FontId, Frame, Layout, RichText, Ui};

pub struct Cell {
    label: String,
    color: Color32,
}

impl Cell {
    pub fn new(label: String, color: Color32) -> Cell {
        Cell { label, color }
    }
    pub fn build(&self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        ui.with_layout(
            Layout::left_to_right(Align::Center).with_main_justify(true),
            |ui| {
                Frame::none()
                    .fill(*&self.color)
                    .rounding(5.0)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(&self.label).font(FontId::proportional(15.0)));
                            add_contents(ui);
                        });
                    });
            },
        );
    }
}
