use glam::Vec2;

use crate::CollisionShape;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    origin: Vec2,
    vector: Vec2,
}

impl Ray {
    fn new(origin: impl Into<[f32; 2]>, vector: impl Into<[f32; 2]>) -> Self {
        Self {
            origin: origin.into().into(),
            vector: vector.into().into(),
        }
    }

    #[allow(clippy::unused_self)]
    fn cast(self, _shape: &CollisionShape) -> Option<[f32; 2]> {
        None
    }
}

mod tests {
    use super::*;
    use crate::{CollisionShape, Transform};
    use rstest::rstest;

    #[rstest]
    #[case(Ray::new([0.0, 0.0], [0.0, 0.0]), CollisionShape::new_circle(1.0).with_transform(Transform::from_translation([1.0, 1.0])))]
    fn returns_none(#[case] ray: Ray, #[case] shape: CollisionShape) {
        assert_eq!(ray.cast(&shape), None);
    }
}
