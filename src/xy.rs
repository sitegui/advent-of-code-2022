#![allow(dead_code)]

use num_traits::{Num, One, Signed, Zero};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, derive_more::Add)]
pub struct Xy<T = i32> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, derive_more::Add)]
pub struct Xyz<T = i32> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Xy<T> {
    pub const fn new(x: T, y: T) -> Self {
        Xy { x, y }
    }
}

impl<T: Num + Signed + Copy> Xy<T> {
    pub fn manhattan_distance(self, other: Self) -> T {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx + dy
    }
}

impl<T> Xyz<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Xyz { x, y, z }
    }
}

impl<T: Zero> Zero for Xy<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: One + Zero> Xy<T> {
    pub fn x_axis() -> Self {
        Xy::new(T::one(), T::zero())
    }

    pub fn y_axis() -> Self {
        Xy::new(T::zero(), T::one())
    }
}

impl<T: One + Zero> Xyz<T> {
    pub fn x_axis() -> Self {
        Xyz::new(T::one(), T::zero(), T::zero())
    }

    pub fn y_axis() -> Self {
        Xyz::new(T::zero(), T::one(), T::zero())
    }

    pub fn z_axis() -> Self {
        Xyz::new(T::zero(), T::zero(), T::one())
    }
}
