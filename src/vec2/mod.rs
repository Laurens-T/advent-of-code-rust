use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Vec2(pub i32, pub i32);

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.0, self.1)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Vec2 {
    pub const UP: Self = Vec2(-1, 0);
    pub const DOWN: Self = Vec2(1, 0);
    pub const LEFT: Self = Vec2(0, -1);
    pub const RIGHT: Self = Vec2(0, 1);
    
    pub fn mul(&self, factor: i32) -> Self {
        Vec2(self.0 * factor, self.1 * factor)
    }
}