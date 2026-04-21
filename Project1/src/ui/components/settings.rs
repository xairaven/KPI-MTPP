use crate::context::Context;
use crate::graphics;
use eframe::epaint::Color32;
use egui::{DragValue, Grid, Panel, RichText, ScrollArea};

#[derive(Debug)]
pub struct SettingsComponent {
    width: f32,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        Self { width: 250.0 }
    }
}

impl SettingsComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        Panel::right("SETTINGS_PANEL")
            .resizable(false)
            .default_size(self.width)
            .min_size(self.width)
            .max_size(self.width)
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading(RichText::new("Settings").color(Color32::WHITE));
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Pixels on Centimeter:");
                        ui.add(
                            DragValue::new(
                                &mut context.viewport.geometry.pixels_per_centimeter,
                            )
                            .speed(1)
                            .range(graphics::PX_PER_CM_RANGE),
                        );

                        ui.vertical_centered_justified(|ui| {
                            if ui.button("Reset").clicked() {
                                context.viewport.geometry.reset_pixels_per_centimeter();
                            }
                        });
                    });

                    Grid::new("AUXILIARY_SETTINGS")
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.checkbox(&mut context.figures.grid.is_enabled, "Grid;");
                            ui.checkbox(
                                &mut context.figures.grid.are_axes_enabled,
                                "Axes;",
                            );
                            ui.checkbox(&mut context.viewport.config.is_pannable, "Pan;");
                            ui.checkbox(&mut context.viewport.config.is_zoomable, "Zoom");

                            ui.end_row();
                        });

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Reset Pan").clicked() {
                            context.viewport.geometry.reset_offset();
                        }
                    });

                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Reset all to defaults").clicked() {
                            context.reset();
                        }
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                });
            });
    }
}
