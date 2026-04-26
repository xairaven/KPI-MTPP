#[derive(Debug)]
pub struct SimulationSettingsUi {
    pub atoms_amount: usize,
    pub time_minutes: usize,
    pub time_seconds: usize,
    pub sampling_times: f64,

    pub probability_up: f64,
    pub probability_down: f64,
    pub probability_left: f64,
    pub probability_right: f64,

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
}

impl Default for SimulationSettingsUi {
    fn default() -> Self {
        Self {
            atoms_amount: 100,
            time_minutes: 0,
            time_seconds: 0,
            sampling_times: 0.1,

            probability_up: 0.0,
            probability_down: 0.0,
            probability_left: 0.0,
            probability_right: 0.0,

            border_width: 10,
            border_height: 10,
        }
    }
}
