use crate::backend::commands::UiCommand;
use crate::utils::channel::Channel;
use egui::{DragValue, emath};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct DragValueNotifiable<'a, Num: emath::Numeric> {
    value: &'a mut Num,
    speed: f64,
    range: RangeInclusive<Num>,
    suffix: String,

    channel: Channel<UiCommand>,
    command: UiCommand,
}

impl<'a, Num: emath::Numeric> DragValueNotifiable<'a, Num> {
    pub fn new(value: &'a mut Num) -> Self {
        Self {
            value,
            speed: 1.0,
            range: Num::MIN..=Num::MAX,
            suffix: "".to_string(),

            channel: Default::default(),
            command: UiCommand::ParameterUpdated,
        }
    }

    pub fn speed(self, speed: impl Into<f64>) -> Self {
        let speed = speed.into();
        Self { speed, ..self }
    }

    pub fn range(self, range: RangeInclusive<Num>) -> Self {
        Self { range, ..self }
    }

    pub fn suffix(self, suffix: &'static str) -> Self {
        let suffix = suffix.to_string();
        Self { suffix, ..self }
    }

    pub fn channel(self, channel: Channel<UiCommand>) -> Self {
        Self { channel, ..self }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        if ui
            .add(
                DragValue::new(self.value)
                    .speed(self.speed)
                    .range(self.range.clone())
                    .suffix(self.suffix.clone()),
            )
            .changed()
        {
            self.channel.try_send(self.command.clone());
        }
    }
}
