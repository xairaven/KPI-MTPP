use crate::backend::crystal::CrystalSize;
use crate::backend::simulation::SimulationSettings;
use crate::graphics::Viewport;
use crate::graphics::primitives::line::Line;
use crate::graphics::primitives::point::Point;
use egui::{Color32, Shape, Stroke};

#[derive(Debug, Default)]
pub struct SimulationVisualizer {
    is_enabled: bool,
    border: Border,
}

impl SimulationVisualizer {
    pub fn start(&mut self, settings: &SimulationSettings) {
        self.is_enabled = true;
        self.border.resize(&settings.crystal_size);
    }

    pub fn stop(&mut self) {
        self.reset();
        self.is_enabled = false;
    }

    pub fn visualize(&self, viewport: &Viewport) -> Vec<Shape> {
        if !self.is_enabled {
            return vec![];
        }

        let mut shapes = Vec::new();

        let border = self
            .border
            .lines()
            .iter()
            .map(|line| line.to_pixels(viewport).to_shape())
            .collect::<Vec<Shape>>();
        shapes.extend(border);

        shapes
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}

#[derive(Debug)]
pub struct Border {
    m: usize,
    n: usize,

    stroke: Stroke,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            m: 10,
            n: 10,

            stroke: Stroke::new(2.0, Color32::BLACK),
        }
    }
}

impl Border {
    pub fn lines(&self) -> Vec<Line<Point>> {
        let s = self.stroke;

        vec![
            Line::with_coordinates(Point::zero(), Point::new(self.m as f64, 0.0))
                .with_stroke(s),
            Line::with_coordinates(Point::zero(), Point::new(0.0, self.n as f64))
                .with_stroke(s),
            Line::with_coordinates(
                Point::new(self.m as f64, 0.0),
                Point::new(self.m as f64, self.n as f64),
            )
            .with_stroke(s),
            Line::with_coordinates(
                Point::new(0.0, self.n as f64),
                Point::new(self.m as f64, self.n as f64),
            )
            .with_stroke(s),
        ]
    }

    pub fn resize(&mut self, size: &CrystalSize) {
        self.m = size.width;
        self.n = size.height;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
