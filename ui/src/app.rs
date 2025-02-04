use egui::CentralPanel;
use top_bar::TopBar;

use crate::views::{home::Home, login::Login, View};

mod top_bar;

#[derive(Debug)]
pub struct App {
    current_view: View,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_view: View::Login,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Default::default()
    }

    pub fn current_view(&self) -> &View {
        &self.current_view
    }

    pub fn navigate(&mut self, view: View) {
        self.current_view = view;
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBar::build(ctx, self);

        CentralPanel::default().show(ctx, |ui| match &self.current_view {
            View::Login => {
                Login::build(ui, self);
            }
            View::Home => {
                Home::build(ui, self);
            }
        });
    }
}
