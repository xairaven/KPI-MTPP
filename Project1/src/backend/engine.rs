use crate::backend::commands::{EngineEvent, UiCommand};
use crate::backend::simulation::Simulation;
use crate::backend::snapshot::CrystalSnapshot;
use crate::ui::modals::error::ErrorModal;
use crossbeam::channel::{Receiver, Sender};
use std::time::Duration;

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

    pub fn run(&mut self) {
        let mut is_running = false;
        let mut current_snapshot_id = 0;

        loop {
            while let Ok(command) = self.ui_commands_rx.try_recv() {
                match command {
                    UiCommand::StartSimulation(settings) => {
                        self.simulation = Some(Simulation::new(settings));
                        is_running = true;
                        current_snapshot_id = 0;
                    },
                    UiCommand::StopSimulation | UiCommand::ParameterUpdated => {
                        is_running = false;
                        current_snapshot_id = 0;
                    },
                }
            }

            if is_running && let Some(simulation) = &mut self.simulation {
                // TODO: Tick

                // Delay
                if simulation.settings.delay_ms > 0 {
                    std::thread::sleep(Duration::from_millis(
                        simulation.settings.delay_ms as u64,
                    ));
                }

                // Sampling
                let elapsed = simulation.start_time.elapsed().as_secs();
                if elapsed as f64 / simulation.settings.sampling_period_seconds
                    > current_snapshot_id as f64
                {
                    let snapshot_data = simulation
                        .crystal
                        .field
                        .iter()
                        .map(|c| c.load(std::sync::atomic::Ordering::Relaxed))
                        .collect();

                    let snapshot = CrystalSnapshot::new(snapshot_data);
                    let _ = self.events_tx.send(EngineEvent::Snapshot(snapshot));
                    current_snapshot_id += 1;
                }
            } else {
                // If thread is on pause, it should sleep to avoid 100% CPU usage
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
}
