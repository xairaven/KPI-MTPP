use crate::backend::crystal::AtomMovementProbability;

#[derive(Debug, Clone)]
pub struct SimulationSettings {
    pub atoms_amount: usize,
    pub time_seconds: usize,
    pub sampling_times: usize,
    pub atom_movement_probability: AtomMovementProbability,
}
