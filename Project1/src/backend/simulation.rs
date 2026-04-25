use crate::backend::crystal::{AtomMovementProbability, Crystal, CrystalSize};
use thiserror::Error;

#[derive(Debug)]
pub struct Simulation {
    pub crystal: Crystal,
    pub settings: SimulationSettings,
}

impl Simulation {
    pub fn new(settings: SimulationSettings) -> Self {
        let crystal = Crystal::new(settings.atoms_amount, settings.crystal_size.clone());

        Self { crystal, settings }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationSettings {
    pub atoms_amount: usize,
    pub time_seconds: usize,
    pub sampling_times: usize,
    pub atom_movement_probability: AtomMovementProbability,
    pub crystal_size: CrystalSize,
}

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Mutex Poisoned.")]
    MutexPoisoned,
}
