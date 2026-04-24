use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Crystal {
    atoms: Vec<Arc<RwLock<Atom>>>,
    field: Vec<AtomicUsize>,

    size: CrystalSize,
}

impl Crystal {
    pub fn new(atoms_amount: usize, size: CrystalSize) -> Self {
        let initial_x = size.width / 2;
        let initial_y = size.height / 2;

        let atoms = (0..atoms_amount)
            .map(|_| {
                Arc::new(RwLock::new(Atom {
                    x: initial_x,
                    y: initial_y,
                }))
            })
            .collect();

        let field = (0..size.width * size.height)
            .map(|_| AtomicUsize::new(0))
            .collect();

        Self { atoms, field, size }
    }
}

impl Default for Crystal {
    fn default() -> Self {
        Crystal::new(
            50,
            CrystalSize {
                width: 10,
                height: 10,
            },
        )
    }
}

#[derive(Debug)]
pub struct Atom {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct AtomMovementProbability {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}

#[derive(Debug, Clone)]
pub struct CrystalSize {
    pub width: usize,
    pub height: usize,
}
