use crate::config::Config;
use crate::graphics::figures::grid::{Grid2D, Grid2DBuilder};
use crate::graphics::units::{Centimeter, Pixel};
use crate::graphics::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};
use crate::simulation::Simulation;
use crate::ui::modals::error::ErrorModal;
use crate::utils::channel::Channel;

#[derive(Debug)]
pub struct Context {
    pub config: Config,

    pub simulation: Simulation,

    pub figures: FiguresState,
    pub viewport: Viewport,

    pub error_modals: Channel<ErrorModal>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            simulation: Simulation::default(),
            figures: FiguresState::default(),
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

            error_modals: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.config.clone());
    }
}

#[derive(Debug)]
pub struct FiguresState {
    pub grid: Grid2D,
}

impl Default for FiguresState {
    fn default() -> Self {
        Self {
            grid: Grid2DBuilder::default().with_unit(Centimeter(1.0)).build(),
        }
    }
}
