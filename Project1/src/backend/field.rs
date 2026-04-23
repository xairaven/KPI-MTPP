#[derive(Debug)]
pub struct Field {
    pub width: usize,
    pub height: usize,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
        }
    }
}
