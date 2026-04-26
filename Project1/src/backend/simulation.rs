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
    pub sampling_times: f64,
    pub seed: Option<u64>,
    pub atom_movement_probability: AtomMovementProbability,
    pub crystal_size: CrystalSize,
}

pub mod ranges {
    use std::ops::RangeInclusive;

    pub const ATOMS_AMOUNT: RangeInclusive<usize> = 1..=100_000;
    pub const TIME: RangeInclusive<usize> = 0..=59;
    pub const SAMPLING: RangeInclusive<f64> = 0.1..=100.0;
    pub const MOVEMENT_PROBABILITY: RangeInclusive<f64> = 0.0..=1.0;
    pub const BORDER: RangeInclusive<usize> = 5..=100;
}

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Mutex Poisoned.")]
    MutexPoisoned,

    #[error(
        "Probabilities must be non-negative and their sum must be less than or equal to 1."
    )]
    BadProbabilities,

    #[error("Seed must be a positive number. {0}")]
    BadSeed(std::num::ParseIntError),
}
