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

fn cast_ray(_origin: Point, vector: Vec2, target: &impl SatShape) -> Option<Contact> {
    let projection = target.project(vector);
    if projection.min > 1.0 {
        return None;
    }
    Some(Contact {
        point: Vec2::new(projection.min, 0.0).into(),
    })
}

#[cfg(test)]
mod tests {
    use crate::v3::shapes::Aabb;

    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        Vec2::ZERO,
        Vec2::X,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(1.5, 0.0)),
        Vec2::new(0.5, 0.0)
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
    fn ray_cast_should_return_none_when_there_is_no_hit() {
        let result = cast_ray(
            Vec2::ZERO.into(),
            Vec2::X,
            &Aabb::from_size(Vec2::new(1.0, 1.0)).with_position(Vec2::new(1.6, 0.0)),
        );
        assert_eq!(result, None);
    }
}
