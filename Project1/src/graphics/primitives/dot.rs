use crate::graphics::Viewport;
use crate::graphics::primitives::point::Point;
use crate::graphics::units::Centimeter;
use eframe::emath::Pos2;
use eframe::epaint::{CircleShape, Color32, Stroke};
use egui::Shape;

#[derive(Debug, Default)]
pub struct Dot {
    pub center: Point,
    pub radius: Centimeter,
    pub fill: Color32,
    pub stroke_color: Color32,
    pub stroke_width: Centimeter,
}

impl Dot {
    pub fn into_shape(self, viewport: &Viewport) -> Shape {
        let circle = CircleShape {
            center: Pos2::from(self.center.to_pixels(viewport)),
            radius: self.radius.to_pixels_vector_x(viewport).0 as f32,
            fill: self.fill,
            stroke: Stroke::new(
                self.stroke_width.to_pixels_vector_x(viewport).0 as f32,
                self.stroke_color,
            ),
        };

        Shape::Circle(circle)
    }
}
