use crate::backend::commands::UiCommand;
use crate::backend::engine::Engine;
use crate::backend::performance::PerformanceMonitor;
use crate::config::Config;
use crate::graphics::figures::grid::{Grid2D, Grid2DBuilder};
use crate::graphics::figures::simulation::SimulationVisualizer;
use crate::graphics::units::{Centimeter, Pixel};
use crate::graphics::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};
use crate::ui::modals::error::ErrorModal;
use crate::ui::states::player::Player;
use crate::ui::states::settings::SimulationSettingsUi;
use crate::utils::channel::Channel;

#[derive(Debug)]
pub struct Context {
    pub engine: Engine,

    pub ui_state: UiState,
    pub performance_monitor: PerformanceMonitor,

    pub viewport: Viewport,

    pub config: Config,

    pub commands_channel: Channel<UiCommand>,
    pub error_modals: Channel<ErrorModal>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        let commands: Channel<UiCommand> = Default::default();
        let errors: Channel<ErrorModal> = Default::default();
        let engine = Engine::new(commands.clone(), errors.clone());
        let player = Player::new(commands.clone());

        Self {
            engine,

            ui_state: UiState::new(player),
            performance_monitor: PerformanceMonitor::new(),

            viewport: Viewport {
                // Default settings like panning, zooming, etc.
                config: Default::default(),
                // Default geometry settings, can be updated by user
                geometry: ViewportGeometry {
                    zero_point_location: ZeroPointLocation::BottomLeftWithOffset {
                        offset: Pixel(50.0),
                    },
                    ..Default::default()
                },
                // Initial viewport state, will be updated when the UI is built
                state: ViewportState::default(),
            },

            config,

            commands_channel: commands,
            error_modals: errors,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.config.clone());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub grid: Grid2D,
    pub player: Player,
    pub simulation_visualizer: SimulationVisualizer,
    pub simulation_settings: SimulationSettingsUi,
}

impl UiState {
    pub fn new(player: Player) -> Self {
        Self {
            player,

            grid: Grid2DBuilder::default().with_unit(Centimeter(1.0)).build(),
            simulation_visualizer: Default::default(),
            simulation_settings: Default::default(),
        }
    }
}
