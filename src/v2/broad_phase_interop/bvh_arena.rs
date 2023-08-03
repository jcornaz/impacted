use bvh_arena::volumes::Aabb;
use glam::Vec2;

use crate::v2::{CollisionShape, Support};

impl From<&CollisionShape> for Aabb<2> {
    fn from(shape: &CollisionShape) -> Self {
        let min = [
            shape.support(Vec2::new(-1.0, 0.0)).x,
            shape.support(Vec2::new(0.0, -1.0)).y,
        ];
        let max = [
            shape.support(Vec2::new(1.0, 0.0)).x,
            shape.support(Vec2::new(0.0, 1.0)).y,
        ];
        Self::from_min_max(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn from_rectangle() {
        let expected = Aabb::from_min_max(Vec2::new(-0.5, -1.0), Vec2::new(0.5, 1.0));
        let actual = Aabb::from(&CollisionShape::new_rectangle(1.0, 2.0));
        assert_eq!(expected, actual);
    }

    #[test]
    fn from_circle() {
        let expected = Aabb::from_min_max(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
        let actual = Aabb::from(&CollisionShape::new_circle(1.0));
        assert_eq!(expected, actual);
    }
}
