use crate::backend::commands::{EngineEvent, UiCommand};
use crate::backend::simulation::Simulation;
use crate::ui::modals::error::ErrorModal;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct Engine {
    pub simulation: Option<Simulation>,

    pub ui_commands_rx: Receiver<UiCommand>,
    pub events_tx: Sender<EngineEvent>,
    pub errors_tx: Sender<ErrorModal>,
}

impl Engine {
    pub fn new(
        commands: Receiver<UiCommand>, events: Sender<EngineEvent>,
        errors: Sender<ErrorModal>,
    ) -> Self {
        Self {
            simulation: None,
            ui_commands_rx: commands,
            events_tx: events,
            errors_tx: errors,
        }
    }

    pub fn run(&mut self) {}
}
