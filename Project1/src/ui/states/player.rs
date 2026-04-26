use crate::backend::commands::UiCommand;
use crate::backend::simulation::SimulationSettings;
use crossbeam::channel::Sender;
use std::time::Instant;

#[derive(Debug)]
pub struct Player {
    pub is_running: bool,
    pub start_time: Option<Instant>,
    pub view_mode: ViewMode,

    pub command_tx: Sender<UiCommand>,
}

impl Player {
    pub fn new(command_tx: Sender<UiCommand>) -> Self {
        Self {
            is_running: false,
            start_time: None,
            view_mode: Default::default(),
            command_tx,
        }
    }

    pub fn start(&mut self, settings: SimulationSettings) {
        if self.view_mode == ViewMode::Snapshot {
            return;
        }

        self.is_running = true;
        self.start_time = Some(Instant::now());
        let _ = self
            .command_tx
            .try_send(UiCommand::StartSimulation(settings));
    }

    pub fn stop(&mut self) {
        if self.view_mode == ViewMode::Snapshot {
            return;
        }
        self.is_running = false;
        self.start_time = None;
        let _ = self.command_tx.try_send(UiCommand::StopSimulation);
    }

    pub fn time(&self) -> String {
        match &self.start_time {
            None => String::from("00 : 00"),
            Some(start_time) => {
                let seconds = start_time.elapsed().as_secs_f64();

                let view_minutes = (seconds / 60.0).floor() as u64;
                let view_seconds = (seconds % 60.0).floor() as u64;

                format!("{:02} : {:02}", view_minutes, view_seconds)
            },
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ViewMode {
    #[default]
    RealTime,
    Snapshot,
}

impl std::fmt::Display for ViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ViewMode::RealTime => write!(f, "Real Time"),
            ViewMode::Snapshot => write!(f, "Snapshot"),
        }
    }
}
