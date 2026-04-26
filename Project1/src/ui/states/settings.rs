use crate::backend::crystal::{AtomMovementProbability, CrystalSize};
use crate::backend::simulation::{SimulationError, SimulationSettings};
use rand::RngExt;

#[derive(Debug, Clone)]
pub struct SimulationSettingsUi {
    pub atoms_amount: usize,
    pub time_minutes: usize,
    pub time_seconds: usize,
    pub delay_ms: usize,
    pub sampling_times: f64,

    pub probability_up: f64,
    pub probability_down: f64,
    pub probability_left: f64,
    pub probability_right: f64,

    pub is_seed_enabled: bool,
    pub seed: String,

    pub border_width: usize,
    pub border_height: usize,
}

impl SimulationSettingsUi {
    pub fn probabilities_sum(&self) -> f64 {
        self.probability_up
            + self.probability_down
            + self.probability_left
            + self.probability_right
    }

    pub fn are_probabilities_valid(&self) -> bool {
        let sum = self.probabilities_sum();

        sum.is_sign_positive() && sum <= 1.0
    }

    pub fn generate_seed(&mut self) {
        let mut rng = rand::rng();
        self.seed = rng.random_range(0..usize::MAX).to_string();
    }
}

impl Default for SimulationSettingsUi {
    fn default() -> Self {
        Self {
            atoms_amount: 100,
            time_minutes: 0,
            time_seconds: 0,
            delay_ms: 0,
            sampling_times: 0.1,

            probability_up: 0.0,
            probability_down: 0.0,
            probability_left: 0.0,
            probability_right: 0.0,

            is_seed_enabled: false,
            seed: String::from("0"),

            border_width: 10,
            border_height: 10,
        }
    }
}

impl TryFrom<SimulationSettingsUi> for SimulationSettings {
    type Error = SimulationError;

    fn try_from(value: SimulationSettingsUi) -> Result<Self, Self::Error> {
        if !value.are_probabilities_valid() {
            return Err(SimulationError::BadProbabilities);
        }
        let seed = if value.is_seed_enabled {
            let seed: u64 = value.seed.parse().map_err(SimulationError::BadSeed)?;
            Some(seed)
        } else {
            None
        };

        Ok(Self {
            atoms_amount: value.atoms_amount,
            time_seconds: value.time_minutes * 60 + value.time_seconds,
            sampling_times: value.sampling_times,
            delay_ms: value.delay_ms,
            seed,
            atom_movement_probability: AtomMovementProbability {
                up: value.probability_up,
                down: value.probability_down,
                left: value.probability_left,
                right: value.probability_right,
            },
            crystal_size: CrystalSize {
                width: value.border_width,
                height: value.border_height,
            },
        })
    }
}
