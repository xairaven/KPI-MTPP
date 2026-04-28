use crate::backend::commands::UiCommand;
use crate::backend::simulation::SimulationSettings;
use crate::graphics::Viewport;
use crate::graphics::figures::border::Border;
use crossbeam::channel::Sender;
use egui::Shape;
use std::time::Instant;

#[derive(Debug)]
pub struct Player {
    pub mode: ViewModeId,
    pub mode_player: ViewMode,
    pub command_tx: Sender<UiCommand>,
}

impl Player {
    pub fn new(command_tx: Sender<UiCommand>) -> Self {
        let mode = ViewModeId::default();
        let mode_player = match &mode {
            ViewModeId::RealTime => {
                ViewMode::RealTime(RealTimeVisualizer::new(command_tx.clone()))
            },
            ViewModeId::Snapshot => ViewMode::Snapshot,
        };

        Self {
            mode,
            mode_player,
            command_tx,
        }
    }

    pub fn change_mode(&mut self) {
        self.mode_player = match self.mode {
            ViewModeId::RealTime => {
                ViewMode::RealTime(RealTimeVisualizer::new(self.command_tx.clone()))
            },
            ViewModeId::Snapshot => ViewMode::Snapshot,
        };
    }

    pub fn visualize(&mut self, viewport: &Viewport) -> Vec<Shape> {
        match &mut self.mode_player {
            ViewMode::Snapshot => {
                todo!()
            },
            ViewMode::RealTime(visualizer) => visualizer.visualize(viewport),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ViewModeId {
    #[default]
    RealTime,
    Snapshot,
}

impl std::fmt::Display for ViewModeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RealTime => write!(f, "Real Time"),
            Self::Snapshot => write!(f, "Snapshot"),
        }
    }
}

#[derive(Debug)]
pub enum ViewMode {
    Snapshot,
    RealTime(RealTimeVisualizer),
}

#[derive(Debug)]
pub struct RealTimeVisualizer {
    pub is_enabled: bool,
    start_time: Option<Instant>,

    border: Border,

    ui_tx: Sender<UiCommand>,
}

impl RealTimeVisualizer {
    pub fn new(ui_tx: Sender<UiCommand>) -> Self {
        Self {
            is_enabled: false,
            start_time: None,
            border: Default::default(),
            ui_tx,
        }
    }

    pub fn start(&mut self, settings: &SimulationSettings) {
        self.is_enabled = true;
        self.start_time = Some(Instant::now());
        self.border.resize(&settings.crystal_size);

        let _ = self
            .ui_tx
            .try_send(UiCommand::StartSimulation(settings.clone()));
    }

    pub fn stop(&mut self) {
        self.reset();
        let _ = self.ui_tx.try_send(UiCommand::StopSimulation);
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

    pub fn visualize(&self, viewport: &Viewport) -> Vec<Shape> {
        if !self.is_enabled {
            return vec![];
        }

        let mut shapes = Vec::new();

        let border = self
            .border
            .lines()
            .iter()
            .map(|line| line.to_pixels(viewport).to_shape())
            .collect::<Vec<Shape>>();
        shapes.extend(border);

        shapes
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.ui_tx.clone());
    }
}
