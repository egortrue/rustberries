pub struct Apple {
    pub position: (usize, usize),
}

impl Apple {
    pub fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }
}
