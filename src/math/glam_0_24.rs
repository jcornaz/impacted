use glam::Vec2;

use super::*;

impl Dot for Vec2 {
    type Scalar = f32;
    fn dot(self, other: Self) -> Self::Scalar {
        Vec2::dot(self, other)
    }
}

impl Perp for Vec2 {
    fn perp(self) -> Self {
        Vec2::perp(self)
    }
}
