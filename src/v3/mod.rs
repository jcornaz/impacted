mod math;
mod range;
mod shapes;

use math::Vec2;
use range::Range;
use shapes::Point;

trait SatShape: AxisProjection {
    type AxisIter: Iterator<Item = Vec2>;
    /// Axes should all be unit vectors
    fn axes(&self) -> Self::AxisIter;
}

trait AxisProjection {
    fn project_on(&self, axis: Vec2) -> Range;
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
struct Contact {
    point: Point,
}

#[cfg(test)]
mod tests {
    use core::mem;

    use crate::v3::shapes::Aabb;

    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    fn cast_ray(origin: Point, vector: Vec2, target: &impl SatShape) -> Option<Contact> {
        let normal = vector.normalize()?;
        let projected_origin = origin.project_on(normal).min;
        let projected_end = (origin + vector).project_on(normal).min;
        let projected_target = target.project_on(normal).min;
        if projected_target < projected_origin
            || projected_target > projected_end
            || !target.project_on(normal.perp()).contains(0.0)
        {
            return None;
        }
        Some(Contact {
            point: (normal * projected_target).into(),
        })
    }

    #[cfg(feature = "std")]
    fn _cast_ray(origin: Point, vector: Vec2, target: &impl SatShape) -> Option<Contact> {
        let mut max_t1 = f32::MIN;
        let mut min_t2 = f32::MAX;
        for axis in target.axes() {
            let proj_vector = vector.dot(axis);
            let proj_origin = origin.project_on(axis).max;
            let proj_target = target.project_on(axis);
            println!("# axis: {axis:?}:");
            println!("vector: {proj_vector:?}");
            println!("origin: {proj_origin:?}");
            println!("target: {proj_target:?}");
            let (t1, t2) = if proj_vector > 0.0 {
                if proj_target.max < proj_origin {
                    println!("early out");
                    return None;
                } else if proj_target.contains(proj_origin) {
                    (0.0, (proj_target.max - proj_origin) / proj_vector)
                } else {
                    (
                        (proj_target.min - proj_origin) / proj_vector,
                        (proj_target.max - proj_origin) / proj_vector,
                    )
                }
            } else if proj_vector < 0.0 {
                if proj_target.min > proj_origin {
                    println!("early out");
                    return None;
                } else if proj_target.contains(proj_origin) {
                    ((proj_target.max - proj_origin) / proj_vector, 0.0)
                } else {
                    (
                        (proj_target.max - proj_origin) / proj_vector,
                        (proj_target.min - proj_origin) / proj_vector,
                    )
                }
            } else {
                (0.0, 0.0)
            };
            println!("t1: {t1}");
            println!("t2: {t2}");
            max_t1 = max_t1.max(t1);
            min_t2 = min_t2.min(t2);
        }
        println!("max_t1: {max_t1}");
        println!("min_t2: {min_t2}");
        println!(
            "{} || {} || {}",
            max_t1 < min_t2,
            max_t1 > 1.0,
            max_t1 <= 0.0
        );
        if max_t1 < min_t2 || max_t1 > 1.0 || max_t1 <= 0.0 {
            println!("no collision");
            return None;
        }
        Some(Contact {
            point: origin + (vector * max_t1),
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
        Vec2::ZERO,
        -Vec2::X,
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_position(Vec2::new(-1.9, 0.0)),
        Vec2::new(-0.9, 0.0)
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
    #[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(1.0, 1.0)).with_position(Vec2::ZERO))]
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

    /// Given ranges of projected shapes,
    /// return the earliest time and latest time there *could* be an impact. (though actual impact is not guarantee)
    ///
    /// Or none, if it is not possible for the source to impact the target.
    fn cast_range(mut source: Range, mut vector: f32, mut target: Range) -> Option<(f32, f32)> {
        if vector == 0.0 {
            return if source.overlaps(target) {
                Some((0.0, f32::MAX))
            } else {
                None
            };
        }
        if vector < 0.0 {
            vector = -vector;
            mem::swap(&mut source, &mut target);
        }
        if source.min > target.max {
            return None;
        }
        Some(if source.max >= target.min {
            (0.0, (target.max - source.min) / vector)
        } else {
            (
                (target.min - source.max) / vector,
                (target.max - source.min) / vector,
            )
        })
    }

    #[rstest]
    #[case((0.0, 0.0), 1.0, (0.5, 1.0), (0.5, 1.0))]
    #[case((0.0, 0.0), 2.0, (0.5, 1.0), (0.25, 0.5))]
    #[case((1.0, 1.0), 1.0, (1.5, 2.0), (0.5, 1.0))]
    #[case((0.0, 1.0), 1.0, (1.5, 2.0), (0.5, 2.0))]
    #[case((0.5, 1.0), -1.0, (0.0, 0.0), (0.5, 1.0))]
    #[case((0.5, 1.0), -2.0, (0.0, 0.0), (0.25, 0.5))]
    #[case((1.5, 2.0), -1.0, (1.0, 1.0), (0.5, 1.0))]
    #[case((1.5, 2.0), -1.0, (0.0, 1.0), (0.5, 2.0))]
    #[case((0.0, 1.0), 1.0, (0.5, 1.5), (0.0, 1.5))]
    #[case((0.0, 1.0), 1.0, (-1.0, 2.0), (0.0, 2.0))]
    #[case((0.0, 1.0), 0.0, (0.5, 1.5), (0.0, f32::MAX))]
    fn cast_range_should_return_some_when_the_shape_are_moving_together(
        #[case] source: (f32, f32),
        #[case] vector: f32,
        #[case] target: (f32, f32),
        #[case] expected: (f32, f32),
    ) {
        let source = Range::from_min_max(source.0, source.1);
        let target = Range::from_min_max(target.0, target.1);
        let (earliest_time, latest_time) = cast_range(source, vector, target).unwrap();
        assert_abs_diff_eq!(earliest_time, expected.0);
        assert_abs_diff_eq!(latest_time, expected.1);
    }

    #[rstest]
    #[case((0.0, 0.0), -1.0, (0.5, 1.0))]
    #[case((0.0, 0.0), -2.0, (0.5, 1.0))]
    #[case((1.0, 1.0), -1.0, (1.5, 2.0))]
    #[case((0.0, 1.0), -1.0, (1.5, 2.0))]
    #[case((0.5, 1.0), 1.0, (0.0, 0.0))]
    #[case((0.5, 1.0), 2.0, (0.0, 0.0))]
    #[case((1.5, 2.0), 1.0, (1.0, 1.0))]
    #[case((1.5, 2.0), 1.0, (0.0, 1.0))]
    #[case((0.0, 0.0), 0.0, (0.5, 1.0))]
    fn cast_range_should_return_none_where_there_cannot_be_a_collistion(
        #[case] source: (f32, f32),
        #[case] vector: f32,
        #[case] target: (f32, f32),
    ) {
        let source = Range::from_min_max(source.0, source.1);
        let target = Range::from_min_max(target.0, target.1);
        assert_eq!(cast_range(source, vector, target), None);
    }
}
