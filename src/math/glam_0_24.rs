use glam::Vec2;

use super::*;

impl Dot for Vec2 {
    type Scalar = f32;
    fn dot(self, other: Self) -> Self::Scalar {
        Vec2::dot(self, other)
    }
}

impl Cross for Vec2 {
    type Output = f32;
    fn cross(self, other: Self) -> Self::Output {
        Vec2::perp_dot(self, other)
    }
}

impl Perp for Vec2 {
    fn perp(self) -> Self {
        Vec2::perp(self)
    }
}

impl Normalize for Vec2 {
    fn normalize(self) -> Option<Self> {
        Vec2::try_normalize(self)
    }
}
