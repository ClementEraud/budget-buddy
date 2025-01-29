use crate::components;

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
        components::top_bar(ctx);

        match &self.current_view {
            View::Login => {
                components::login_view(ctx, self);
            }
            View::Home => {
                components::home_view(ctx, self);
            }
        }
    }
}
