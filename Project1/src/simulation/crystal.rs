use crate::graphics::figures::border::Border;

#[derive(Debug)]
pub struct Crystal {}

impl Crystal {
    pub fn new(border: &Border) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Atom {
    pub x: usize,
    pub y: usize,
}
