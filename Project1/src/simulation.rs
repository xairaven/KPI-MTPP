use crate::graphics::figures::border::Border;
use crate::simulation::crystal::Crystal;

#[derive(Debug)]
pub struct Simulation {
    pub crystal: Crystal,
    pub border: Border,
}

impl Default for Simulation {
    fn default() -> Self {
        let border = Default::default();

        Self {
            crystal: Crystal::new(&border),
            border,
        }
    }
}

pub mod crystal;
