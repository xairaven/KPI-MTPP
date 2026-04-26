use crate::backend::commands::UiCommand;
use crate::backend::simulation::{Simulation, SimulationSettings};
use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;

#[derive(Debug)]
pub struct Engine {
    pub simulation: Option<Simulation>,

    pub commands_channel: Channel<UiCommand>,
    pub errors_channel: Channel<ErrorModal>,
}

impl Engine {
    pub fn new(commands: Channel<UiCommand>, errors: Channel<ErrorModal>) -> Self {
        Self {
            simulation: None,
            commands_channel: commands,
            errors_channel: errors,
        }
    }

    pub fn start_simulation(&mut self, settings: SimulationSettings) {
        self.simulation = Some(Simulation::new(settings));
    }

    pub fn stop_simulation(&mut self) {
        self.simulation = None;
    }
}
