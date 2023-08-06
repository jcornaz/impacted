mod math;
mod range;
mod shapes;

use math::Vec2;
use range::Range;

trait SatShape: AxisProjection {
    type AxisIter: Iterator<Item = Vec2>;
    fn axes(&self) -> Self::AxisIter;
}

trait AxisProjection {
    fn project(&self, axis: Vec2) -> Range;
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
struct Contact {}

fn cast_ray(_origin: Vec2, _vector: Vec2, _target: &impl SatShape) -> Option<Contact> {
    None
}

#[cfg(test)]
mod tests {
    use crate::v3::shapes::Aabb;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn ray_cast_should_return_none_when_there_is_no_hit() {
        let result = cast_ray(
            Vec2::ZERO,
            Vec2::X,
            &Aabb::from_size(Vec2::new(1.0, 1.0)).with_position(Vec2::X * 2.0),
        );
        assert_eq!(result, None);
    }
}
