#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Location {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}
