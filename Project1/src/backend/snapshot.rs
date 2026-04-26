use crate::backend::crystal::{Atom, Crystal};
use crate::backend::simulation::SimulationError;
use std::sync::atomic::Ordering;

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
    pub atoms: Vec<Atom>,
    pub field: Vec<usize>,
    pub total_atoms: usize,
}

impl TryFrom<&Crystal> for CrystalSnapshot {
    type Error = SimulationError;

    fn try_from(crystal: &Crystal) -> Result<Self, Self::Error> {
        let mut atoms = Vec::with_capacity(crystal.atoms.len());
        for atom in &crystal.atoms {
            atoms.push(atom.clone());
        }

        let mut field = Vec::with_capacity(crystal.field.len());
        for cell in &crystal.field {
            let value = cell.load(Ordering::Relaxed);
            field.push(value);
        }

        let total_atoms = field.iter().sum();

        Ok(Self {
            atoms,
            field,
            total_atoms,
        })
    }
}
