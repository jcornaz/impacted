mod math;
mod range;
mod shapes;

use math::Vec2;
use range::Range;
use shapes::Point;

trait SatShape: AxisProjection {
    type AxisIter: Iterator<Item = Vec2>;
    fn axes(&self) -> Self::AxisIter;
}

trait AxisProjection {
    fn project(&self, axis: Vec2) -> Range;
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
struct Contact {
    point: Point,
}

#[cfg(test)]
mod tests {
    use crate::v3::shapes::Aabb;

    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    fn cast_ray(origin: Point, vector: Vec2, target: &impl SatShape) -> Option<Contact> {
        let normal = vector.normalize()?;
        let projected_origin = origin.project(normal).min;
        let projected_end = (origin + vector).project(normal).min;
        let projected_target = target.project(normal).min;
        if projected_target < projected_origin
            || projected_target > projected_end
            || !target.project(normal.perp()).contains(0.0)
        {
            return None;
        }
        Some(Contact {
            point: (normal * projected_target).into(),
        })
    }

    #[rstest]
    #[case(
        Vec2::ZERO,
        Vec2::X,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.9, 0.0)),
        Vec2::new(0.9, 0.0)
    )]
    #[case(
        Vec2::X,
        Vec2::X,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(2.9, 0.0)),
        Vec2::new(1.9, 0.0)
    )]
    #[case(
        Vec2::ZERO,
        Vec2::X * 2.0,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(2.9, 0.0)),
        Vec2::new(1.9, 0.0)
    )]
    #[case(
        Vec2::ZERO,
        Vec2::Y,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(0.0, 1.9)),
        Vec2::new(0.0, 0.9)
    )]
    #[case(
        Vec2::Y,
        Vec2::Y,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(0.0, 2.9)),
        Vec2::new(0.0, 1.9)
    )]
    #[case(
        Vec2::ZERO,
        Vec2::Y * 2.0,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(0.0, 2.9)),
        Vec2::new(0.0, 1.9)
    )]
    #[case(
        Vec2::ZERO,
        Vec2::new(1.0, 1.0),
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.9, 1.9)),
        Vec2::new(0.9, 0.9),
    )]
    #[ignore = "not implemented"]
    #[case(
        Vec2::ZERO,
        Vec2::new(1.0, 1.0),
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(0.5, 1.9)),
        Vec2::new(0.9, 0.9),
    )]
    fn ray_cast_should_find_contact_point(
        #[case] origin: impl Into<Point>,
        #[case] vector: Vec2,
        #[case] target: impl SatShape,
        #[case] expected_point: impl Into<Point>,
    ) {
        let point = cast_ray(origin.into(), vector, &target).unwrap().point;
        assert_abs_diff_eq!(point, expected_point.into());
    }

    #[rstest]
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(2.1, 0.0)))]
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(-2.1, 0.0)))]
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::ZERO))]
    #[case(-Vec2::X, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.1, 0.0)))]
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.9, 5.0)))]
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.9, -5.0)))]
    fn ray_cast_should_return_none_when_there_is_no_hit(
        #[case] origin: impl Into<Point>,
        #[case] vector: Vec2,
        #[case] target: impl SatShape,
    ) {
        let result = cast_ray(origin.into(), vector, &target);
        assert_eq!(result, None);
    }
}
