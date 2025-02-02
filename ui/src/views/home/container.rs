use balance::Balance;
use egui::{Color32, FontId, Frame, Grid, RichText, Stroke, Ui, Vec2};
use expenses::Expenses;
use incomes::Incomes;

mod balance;
mod cell;
mod expenses;
mod incomes;

pub struct Container {
    label: String,
}

impl Container {
    pub fn new(label: String) -> Container {
        Container { label }
    }

    pub fn build(&self, ui: &mut Ui) {
        Frame::none()
            .stroke(Stroke::new(1.0, Color32::BLACK))
            .rounding(5.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(RichText::new(&self.label).font(FontId::proportional(20.0)));
                    ui.add_space(20.0);

                    Grid::new(format!("container {}", &self.label))
                        .max_col_width(ui.available_width() / 3.0)
                        .min_row_height(ui.available_height())
                        .spacing(Vec2 { x: 10.0, y: 0.0 })
                        .num_columns(3)
                        .show(ui, |ui| {
                            Incomes::build(ui);
                            Expenses::build(ui);
                            Balance::build(ui);
                        });

                    ui.add_space(ui.available_height() - 30.0);
                });
            });
    }
}
