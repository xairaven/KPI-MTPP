use crate::graphics::primitives::line::Line;
use crate::graphics::primitives::point::Point;
use egui::{Color32, Stroke};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Border {
    pub n: usize,
    pub m: usize,

    stroke: Stroke,
    updated: bool,
}

pub const BORDER_RANGE: RangeInclusive<usize> = 5..=100;

impl Default for Border {
    fn default() -> Self {
        Self {
            n: 10,
            m: 10,

            stroke: Stroke::new(2.0, Color32::BLACK),
            updated: false,
        }
    }
}

impl Border {
    pub fn lines(&self) -> Vec<Line<Point>> {
        let s = self.stroke;

        vec![
            Line::with_coordinates(Point::zero(), Point::new(self.n as f64, 0.0))
                .with_stroke(s),
            Line::with_coordinates(Point::zero(), Point::new(0.0, self.m as f64))
                .with_stroke(s),
            Line::with_coordinates(
                Point::new(self.n as f64, 0.0),
                Point::new(self.n as f64, self.m as f64),
            )
            .with_stroke(s),
            Line::with_coordinates(
                Point::new(0.0, self.m as f64),
                Point::new(self.n as f64, self.m as f64),
            )
            .with_stroke(s),
        ]
    }

    pub fn set_updated(&mut self) {
        self.updated = true;
    }

    pub fn notify_when_updated(&mut self) -> bool {
        let updated = self.updated;
        self.updated = false;

        updated
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
