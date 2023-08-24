#![allow(missing_docs)]

#[cfg(feature = "unstable-v3-aabb")]
pub use aabb::Aabb;
pub use point::Point;
use range::Range;
use sealed::sealed;
pub use vector::Vec2;

#[cfg(feature = "unstable-v3-aabb")]
mod aabb;
mod interop;
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

#[sealed]
pub trait Collides<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

#[sealed]
impl<A, B> Collides<B> for A
where
    A: Shape,
    B: Shape,
{
    fn collides(&self, other: &B) -> bool {
        for axis in sat_axes(self, other) {
            let r1 = self.project_on(axis);
            let r2 = other.project_on(axis);
            if !r1.overlaps(r2) {
                return false;
            }
        }
        true
    }
}

#[sealed]
pub trait Cast<Rhs> {
    fn cast(&self, vector: impl Into<Vec2>, target: &Rhs) -> Option<CastHit>;
}

#[sealed]
impl<A, B> Cast<B> for A
where
    A: Shape,
    B: Shape,
{
    fn cast(&self, vector: impl Into<Vec2>, target: &B) -> Option<CastHit> {
        let vector = vector.into();
        let mut max_t1 = f32::MIN;
        let mut min_t2 = f32::MAX;
        let mut normal = Vec2::ZERO;
        for axis in sat_axes(self, target) {
            let Some((t1, t2)) = cast_projection(
                self.project_on(axis),
                vector.dot(axis),
                target.project_on(axis),
            ) else {
                return None;
            };
            if t1 > max_t1 {
                max_t1 = t1;
                normal = axis;
            }
            max_t1 = max_t1.max(t1);
            min_t2 = min_t2.min(t2);
        }
        if min_t2 < max_t1 || max_t1 > 1.0 || max_t1 <= 0.0 {
            return None;
        }
        if normal.dot(vector) > 0.0 {
            normal = -normal;
        }
        Some(CastHit {
            time: max_t1,
            normal,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct CastHit {
    pub time: f32,
    pub normal: Vec2,
}

pub fn ray_cast(origin: Point, vector: impl Into<Vec2>, target: &impl Shape) -> Option<CastHit> {
    origin.cast(vector, target)
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
