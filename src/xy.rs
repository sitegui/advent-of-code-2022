use num_traits::{Num, Signed};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct XY<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Signed + Copy> XY<T> {
    pub fn new(x: T, y: T) -> Self {
        XY { x, y }
    }

    pub fn manhattan_distance(self, other: Self) -> T {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx + dy
    }
}
