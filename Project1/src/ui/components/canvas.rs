use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense};

#[derive(Debug, Default)]
pub struct CanvasComponent;

impl CanvasComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        CentralPanel::default().show_inside(ui, |ui| {
            Frame::canvas(ui.style())
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    Self::pipeline(ui, context);
                });
        });
    }

    fn pipeline(ui: &mut egui::Ui, context: &mut Context) -> Response {
        Self::draw(ui, context)
    }

    fn draw(ui: &mut egui::Ui, context: &mut Context) -> Response {
        let (response, _painter) = Self::initialize_painter(ui, context);
        // painter.extend(..);

        response
    }

    fn initialize_painter(
        ui: &mut egui::Ui, _context: &mut Context,
    ) -> (Response, Painter) {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) =
            ui.allocate_painter(painter_size, Sense::click_and_drag());

        (response, painter)
    }
}
