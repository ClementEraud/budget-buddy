mod login_view;
mod logout_view;
mod top_bar;

use login_view::LoginView;
use logout_view::LogoutView;

use crate::top_bar::TopBar;

#[derive(Debug)]
pub enum View {
    Login,
    Home,
}

pub struct TemplateApp {
    pub current_view: View,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            current_view: View::Login,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBar::build(ctx);

        match &self.current_view {
            View::Login => {
                LoginView::build(ctx, self);
            }
            View::Home => {
                LogoutView::build(ctx, self);
            }
        }
    }
}
