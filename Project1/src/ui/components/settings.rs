use crate::backend::commands::UiCommand::ParameterUpdated;
use crate::context::Context;
use crate::graphics;
use crate::graphics::figures::border;
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

                    self.border_settings(ui, context);

                    self.separator(ui);

                    self.performance(ui, context);

                    self.separator(ui);

                    self.ui_settings(ui, context);
                });
            });
    }

    fn performance(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            self.header(ui, "System Performance");
        });

        let metrics = &context.performance_monitor.current_metrics;

        ui.label(format!(
            "CPU Global Usage: {:.1}%",
            metrics.global_cpu_usage
        ));
        ui.label(format!(
            "RAM: {:.0} / {:.0} MB",
            metrics.memory_used_mb, metrics.memory_total_mb
        ));

        ui.collapsing("CPUs Info", |ui| {
            Grid::new("CPUs").num_columns(2).show(ui, |ui| {
                for unit in &metrics.cpus_info {
                    ui.label(&unit.name);
                    ui.label(format!("{:.1}%", unit.usage));
                    ui.end_row();
                }
            })
        });
    }

    fn ui_settings(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            self.header(ui, "UI Settings");
        });

        ui.horizontal(|ui| {
            ui.label("Pixels on Centimeter:");
            ui.add(
                DragValue::new(&mut context.viewport.geometry.pixels_per_centimeter)
                    .speed(1)
                    .range(graphics::PX_PER_CM_RANGE),
            );

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset").clicked() {
                    context.viewport.geometry.reset_pixels_per_centimeter();
                }
            });
        });

        Grid::new("UI_GRID_SETTINGS").num_columns(2).show(ui, |ui| {
            ui.checkbox(&mut context.ui_state.grid.is_enabled, "Grid;");
            ui.checkbox(&mut context.ui_state.grid.are_axes_enabled, "Axes;");
            ui.end_row();
        });

        Grid::new("PAN_ZOOM_SETTINGS")
            .num_columns(2)
            .show(ui, |ui| {
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
    }

    fn border_settings(&self, ui: &mut egui::Ui, context: &mut Context) {
        self.header(ui, "Border");

        let border = &mut context.ui_state.border;
        let commands_channel = &context.commands_channel;

        Grid::new("BORDER_RANGE").num_columns(4).show(ui, |ui| {
            ui.label("Range");
            ui.end_row();

            ui.label("M:");
            if ui
                .add(
                    DragValue::new(&mut border.m)
                        .speed(1)
                        .range(border::BORDER_RANGE),
                )
                .changed()
            {
                commands_channel.try_send(ParameterUpdated);
            }

            ui.label("N:");
            if ui
                .add(
                    DragValue::new(&mut border.n)
                        .speed(1)
                        .range(border::BORDER_RANGE),
                )
                .changed()
            {
                commands_channel.try_send(ParameterUpdated);
            }
            ui.end_row();
        });

        ui.add_space(5.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
                border.reset();
                commands_channel.try_send(ParameterUpdated);
            }
        });
    }

    fn separator(&self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
    }

    fn header(&self, ui: &mut egui::Ui, header: &str) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new(header).size(14.0));
        });

        ui.add_space(5.0);
    }
}
