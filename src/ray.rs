use crate::CollisionShape;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray<V> {
    origin: V,
    vector: V,
}

impl<V> Ray<V> {
    fn new(origin: V, vector: V) -> Self {
        Self { origin, vector }
    }

    #[allow(clippy::unused_self)]
    fn cast(self, _shape: &CollisionShape) -> Option<V> {
        None
    }
}

mod tests {
    use rstest::rstest;

    use crate::{CollisionShape, Transform};

    type Vector = [f32; 2];
    type Ray = super::Ray<Vector>;

    #[rstest]
    #[case(Ray::new([0.0, 0.0], [0.0, 0.0]), CollisionShape::new_circle(1.0).with_transform(Transform::from_translation([1.0, 1.0])))]
    fn returns_none(#[case] ray: Ray, #[case] shape: CollisionShape) {
        assert_eq!(ray.cast(&shape), None);
    }
}
