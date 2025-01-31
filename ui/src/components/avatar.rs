use egui::{epaint::CircleShape, Color32, ImageSource, Rect, Stroke, Ui, Vec2};

pub struct Avatar<'a> {
    img_src: ImageSource<'a>,
}

impl<'a> Avatar<'a> {
    pub fn new(img_src: ImageSource<'a>) -> Avatar<'a> {
        Avatar { img_src }
    }

    pub fn build(self, ui: &mut Ui) {
        // Create a circle
        let available_rect = ui.available_rect_before_wrap();
        let image_pos = egui::pos2(
            available_rect.min.x + available_rect.width() / 2.0,
            available_rect.min.y + available_rect.height() / 3.0,
        );
        let radius = 60.0;
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
        let image_size = Vec2 { x: 100.0, y: 50.0 };
        let rect = Rect::from_center_size(image_pos, image_size);
        egui::Image::new(self.img_src.clone())
            .tint(Color32::LIGHT_GRAY)
            .paint_at(ui, rect);
    }
}
