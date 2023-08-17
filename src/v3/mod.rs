#![allow(missing_docs)]

use sealed::sealed;

pub use math::Vec2;
use range::Range;
pub use shapes::{Aabb, Point};

mod math;
mod range;
mod shapes;

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

#[cfg(all(test, feature = "std"))]
fn check_collision(_: &impl Shape, _: &impl Shape) -> bool {
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
    for axis in target.axes() {
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

#[cfg(test)]
#[cfg(feature = "std")]
mod collision_spec {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        Aabb::from_size(Vec2::new(2.0, 2.0)),
        Aabb::from_size(Vec2::new(2.0, 2.0))
    )]
    #[case(
        Aabb::from_size(Vec2::new(2.0, 2.0)),
        Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 0.0))
    )]
    fn test_collides(#[case] shape1: impl Shape, #[case] shape2: impl Shape) {
        assert!(check_collision(&shape1, &shape2));
    }
}

#[cfg(test)]
mod test {
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
}
