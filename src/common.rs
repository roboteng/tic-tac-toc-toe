#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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

impl From<(usize, usize, usize)> for Location {
    fn from(l: (usize, usize, usize)) -> Self {
        Self::new(l.0, l.1, l.2)
    }
}
