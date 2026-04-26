use crate::context::Context;
use egui::{CentralPanel, Color32, Frame, Painter, Response, Sense, Shape};

#[derive(Debug, Default)]
pub struct CanvasComponent;

impl CanvasComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        CentralPanel::default().show_inside(ui, |ui| {
            Frame::canvas(ui.style())
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.input(|i| {
                        context.viewport.handle_scroll(i);
                    });
                    let response = Self::pipeline(ui, context);
                    context.viewport.handle_pan(ui, response);
                });
        });
    }

    fn pipeline(ui: &mut egui::Ui, context: &mut Context) -> Response {
        let shapes = Self::create_shapes(ui, context);
        Self::draw(ui, context, shapes)
    }

    fn create_shapes(_ui: &mut egui::Ui, context: &mut Context) -> Vec<Shape> {
        let mut shapes = vec![];

        let grid: Vec<Shape> = context
            .ui_state
            .grid
            .lines(&context.viewport)
            .iter()
            .map(|line| line.to_pixels(&context.viewport).to_shape())
            .collect::<Vec<Shape>>();
        shapes.extend(grid);

        let simulation: Vec<Shape> = context
            .ui_state
            .simulation_visualizer
            .visualize(&context.viewport);
        shapes.extend(simulation);

        shapes
    }

    fn draw(ui: &mut egui::Ui, context: &mut Context, shapes: Vec<Shape>) -> Response {
        let (response, painter) = Self::initialize_painter(ui, context);
        painter.extend(shapes);

        response
    }

    fn initialize_painter(
        ui: &mut egui::Ui, context: &mut Context,
    ) -> (Response, Painter) {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) =
            ui.allocate_painter(painter_size, Sense::click_and_drag());

        context.viewport.update_state(&response);

        (response, painter)
    }
}
