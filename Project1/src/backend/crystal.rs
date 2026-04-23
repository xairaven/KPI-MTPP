use crate::backend::field::Field;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Crystal {
    vec: Vec<Arc<RwLock<Atom>>>,
    field: Field,
}

impl Crystal {
    pub fn new(field: Field) -> Self {
        let x = field.width / 2;
        let y = field.height / 2;

        Self {
            field,
            vec: (0..(x * y))
                .map(|_| Arc::new(RwLock::new(Atom { x, y })))
                .collect(),
        }
    }

    pub fn index(&self, x: usize, y: usize) -> Option<&Arc<RwLock<Atom>>> {
        self.vec.get(self.field.width * y + x)
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }
}

#[derive(Debug)]
pub struct Atom {
    pub x: usize,
    pub y: usize,
}
