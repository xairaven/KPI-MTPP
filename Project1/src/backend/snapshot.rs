use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct SnapshotStorage {
    real_time_snaps: VecDeque<CrystalSnapshot>,
    user_snaps: Vec<CrystalSnapshot>,
    current: Option<usize>,
}

impl SnapshotStorage {
    pub fn last_realtime(&mut self) -> Option<CrystalSnapshot> {
        let result = self.real_time_snaps.pop_back();

        self.real_time_snaps.clear();

        result
    }

    pub fn add_user_snapshot(&mut self, crystal: CrystalSnapshot) {
        self.user_snaps.push(crystal);
    }

    pub fn current_user_snapshot(&self) -> Option<&CrystalSnapshot> {
        let current = self.current?;

        self.user_snaps.get(current)
    }

    pub fn clear_user_snapshots(&mut self) {
        self.current = None;
        self.user_snaps.clear();
    }
}

#[derive(Debug, Clone)]
pub struct CrystalSnapshot {
    pub field: Vec<usize>,
    pub total_atoms: usize,
}

impl CrystalSnapshot {
    pub fn new(field: Vec<usize>) -> Self {
        Self {
            total_atoms: field.iter().sum(),
            field,
        }
    }
}
