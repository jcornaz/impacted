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
        let projection = target.project(vector).min;
        if !(0.0..=1.0).contains(&projection) {
            return None;
        }
        Some(Contact {
            point: Vec2::new(projection + origin.project(vector).min, 0.0).into(),
        })
    }

    #[rstest]
    #[case(
        Vec2::ZERO,
        Vec2::X,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.5, 0.0)),
        Vec2::new(0.5, 0.0)
    )]
    #[case(
        Vec2::new(10.0, 0.0),
        Vec2::X,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.5, 0.0)),
        Vec2::new(10.5, 0.0)
    )]
    #[ignore = "not implemented"]
    #[case(
        Vec2::ZERO,
        Vec2::X * 2.0,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(2.5, 0.0)),
        Vec2::new(1.5, 0.0)
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
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(1.0, 1.0)).with_position(Vec2::new(1.6, 0.0)))]
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(1.0, 1.0)).with_position(Vec2::new(-1.6, 0.0)))]
    #[ignore = "not implemented"]
    #[case(Vec2::X * 10.0, Vec2::X, Aabb::from_size(Vec2::new(1.0, 1.0)).with_position(Vec2::new(0.5, 0.0)))]
    fn ray_cast_should_return_none_when_there_is_no_hit(
        #[case] origin: impl Into<Point>,
        #[case] vector: Vec2,
        #[case] target: impl SatShape,
    ) {
        let result = cast_ray(origin.into(), vector, &target);
        assert_eq!(result, None);
    }
}
