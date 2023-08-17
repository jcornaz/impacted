#![allow(missing_docs)]

use sealed::sealed;

#[cfg(feature = "unstable-v3-aabb")]
pub use aabb::Aabb;
pub use point::Point;
use range::Range;
pub use vector::Vec2;

#[cfg(feature = "unstable-v3-aabb")]
mod aabb;
mod point;
mod range;
mod vector;

#[sealed]
pub trait Shape {
    type AxisIter: Iterator<Item = Vec2>;
    type FocalsIter: Iterator<Item = Point>;
    type VerticesIter: Iterator<Item = Point>;

    fn axes(&self) -> Self::AxisIter;
    fn focals(&self) -> Self::FocalsIter;
    fn vertices(&self) -> Self::VerticesIter;

    fn project_on(&self, axis: Vec2) -> Range;
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Contact {
    pub point: Point,
}

/// Given ranges of projected shapes,
/// return the earliest time and latest time there *could* be an impact. (though actual impact is not guarantee)
///
/// Or none, if it is not possible for the source to impact the target.
fn cast_projection(mut source: Range, mut vector: f32, mut target: Range) -> Option<(f32, f32)> {
    if vector == 0.0 {
        return if source.overlaps(target) {
            Some((0.0, f32::MAX))
        } else {
            None
        };
    }
    if vector < 0.0 {
        vector = -vector;
        core::mem::swap(&mut source, &mut target);
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

pub fn check_collision(shape1: &impl Shape, shape2: &impl Shape) -> bool {
    for axis in sat_axes(shape1, shape2) {
        let r1 = shape1.project_on(axis);
        let r2 = shape2.project_on(axis);
        if !r1.overlaps(r2) {
            return false;
        }
    }
    true
}

pub fn cast_ray(origin: Point, vector: Vec2, target: &impl Shape) -> Option<Contact> {
    let time = contact_time(&origin, vector, target)?;
    Some(Contact {
        point: origin + (vector * time),
    })
}

fn contact_time(origin: &impl Shape, vector: Vec2, target: &impl Shape) -> Option<f32> {
    let mut max_t1 = f32::MIN;
    let mut min_t2 = f32::MAX;
    for axis in sat_axes(origin, target) {
        let Some((t1, t2)) = cast_projection(
            origin.project_on(axis),
            vector.dot(axis),
            target.project_on(axis),
        ) else { return None };
        max_t1 = max_t1.max(t1);
        min_t2 = min_t2.min(t2);
    }
    if min_t2 < max_t1 || max_t1 > 1.0 || max_t1 <= 0.0 {
        return None;
    }
    Some(max_t1)
}

fn sat_axes(shape1: &impl Shape, shape2: &impl Shape) -> impl Iterator<Item = Vec2> {
    shape1.axes().chain(shape2.axes())
}

#[cfg(test)]
mod test {
    #[cfg(feature = "unstable-v3-aabb")]
    use alloc::vec::Vec;
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    use super::*;

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
        let (earliest_time, latest_time) = cast_projection(source, vector, target).unwrap();
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
    fn cast_range_should_return_none_where_there_cannot_be_a_collision(
        #[case] source: (f32, f32),
        #[case] vector: f32,
        #[case] target: (f32, f32),
    ) {
        let source = Range::from_min_max(source.0, source.1);
        let target = Range::from_min_max(target.0, target.1);
        assert_eq!(cast_projection(source, vector, target), None);
    }

    #[test]
    #[cfg(feature = "unstable-v3-aabb")]
    fn sat_axes_aabb_to_aabb() {
        let actual: Vec<Vec2> = sat_axes(
            &Aabb::from_size(Vec2::splat(1.0)),
            &Aabb::from_size(Vec2::splat(2.0)),
        )
        .collect();
        assert_eq!(&actual[..], &[Vec2::X, Vec2::Y, Vec2::X, Vec2::Y]);
    }

    #[test]
    #[cfg(feature = "unstable-v3-aabb")]
    fn sat_axes_point_to_aabb() {
        let v1: Vec<Vec2> = sat_axes(&Point::ORIGIN, &Aabb::from_size(Vec2::splat(2.0))).collect();
        let v2: Vec<Vec2> = sat_axes(&Aabb::from_size(Vec2::splat(2.0)), &Point::ORIGIN).collect();
        assert_eq!(&v1[..], &[Vec2::X, Vec2::Y]);
        assert_eq!(&v2[..], &[Vec2::X, Vec2::Y]);
    }
}
