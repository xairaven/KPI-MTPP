#[derive(Debug, Default)]
pub struct SnapshotStorage {
    buffer: Vec<CrystalSnapshot>,
    pub current: Option<usize>,
}

impl SnapshotStorage {
    pub fn add(&mut self, crystal: CrystalSnapshot) {
        self.buffer.push(crystal);
    }

    pub fn current(&self) -> Option<&CrystalSnapshot> {
        let current = self.current?;

        self.buffer.get(current)
    }

    pub fn clear(&mut self) {
        self.current = None;
        self.buffer.clear();
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
