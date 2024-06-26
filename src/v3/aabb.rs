use core::iter;

use sealed::sealed;

use super::{__seal_shape, vector::Vec2, Point, Range, Shape};

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    center: Point,
    half_size: Vec2,
}

impl Aabb {
    #[must_use]
    pub fn from_size(size: impl Into<Vec2>) -> Self {
        Self {
            center: Point::ORIGIN,
            half_size: size.into() / 2.0,
        }
    }

    pub fn set_center_at(&mut self, center: impl Into<Point>) {
        self.center = center.into();
    }

    #[must_use]
    pub fn with_center_at(mut self, center: impl Into<Point>) -> Self {
        self.set_center_at(center);
        self
    }
}

#[sealed]
impl Shape for Aabb {
    fn axes(&self) -> impl Iterator<Item = Vec2> {
        [Vec2::X, Vec2::Y].into_iter()
    }

    fn focals(&self) -> impl Iterator<Item = Point> {
        iter::empty()
    }

    fn vertices(&self) -> impl Iterator<Item = Point> {
        [
            (self.center - self.half_size),
            (self.center + Vec2::new(self.half_size.x, -self.half_size.y)),
            (self.center + self.half_size),
            (self.center + Vec2::new(-self.half_size.x, self.half_size.y)),
        ]
        .into_iter()
    }

    fn project_on(&self, axis: Vec2) -> Range {
        let r1 = abs(self.half_size.dot(axis));
        let r2 = abs(Vec2::new(-self.half_size.x, self.half_size.y).dot(axis));
        let r = r1.max(r2);
        let shift = Vec2::from(self.center).dot(axis);
        Range::from_min_max(shift - r, shift + r)
    }
}

#[cfg(feature = "std")]
fn abs(v: f32) -> f32 {
    v.abs()
}

#[cfg(not(feature = "std"))]
fn abs(v: f32) -> f32 {
    libm::fabsf(v)
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Aabb::from_size(Vec2::new(0.0, 0.0)), Vec2::new(1.0, 0.0), 0.0, 0.0)]
    #[case(Aabb::from_size(Vec2::new(2.0, 0.0)), Vec2::new(1.0, 0.0), -1.0, 1.0)]
    #[case(Aabb::from_size(Vec2::new(2.0, 0.0)), Vec2::new(-1.0, 0.0), -1.0, 1.0)]
    #[case(Aabb::from_size(Vec2::new(0.0, 2.0)), Vec2::new(0.0, 1.0), -1.0, 1.0)]
    #[case(Aabb::from_size(Vec2::new(0.0, 2.0)), Vec2::new(0.0, -1.0), -1.0, 1.0)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(1.0, 0.0), -1.5, 1.5)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(0.0, 1.0), -2.0, 2.0)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(1.5, 2.0), -6.25, 6.25)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(1.5, -2.0), -6.25, 6.25)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(-1.5, 2.0), -6.25, 6.25)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_center_at(Point::new(0.0, 0.0)), Vec2::new(1.0, 0.0), -1.5, 1.5)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_center_at(Point::new(1.0, 0.0)), Vec2::new(1.0, 0.0), -0.5, 2.5)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_center_at(Point::new(0.0, 1.0)), Vec2::new(1.0, 0.0), -1.5, 1.5)]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_center_at(Point::new(0.0, 1.0)), Vec2::new(0.0, 1.0), -1.0, 3.0)]
    fn test_axis_project(
        #[case] shape: Aabb,
        #[case] axis: Vec2,
        #[case] expected_min: f32,
        #[case] expected_max: f32,
    ) {
        let range = shape.project_on(axis);
        assert_abs_diff_eq!(range.min, expected_min);
        assert_abs_diff_eq!(range.max, expected_max);
    }

    #[rstest]
    fn test_sat_axes(
        #[values(
        Aabb::from_size(Vec2::ZERO),
        Aabb::from_size(Vec2::new(2.0, 3.0)).with_center_at(Point::new(4.0, 5.0))
        )]
        shape: Aabb,
    ) {
        let mut iterator = shape.axes();
        assert_eq!(iterator.next(), Some(Vec2::X));
        assert_eq!(iterator.next(), Some(Vec2::Y));
        assert!(iterator.next().is_none(), "too many axes returned");
    }

    #[test]
    fn test_sat_vertices() {
        let aabb = Aabb::from_size(Vec2::new(2.0, 3.0)).with_center_at(Point::new(4.0, 5.0));
        let vertices: Vec<_> = aabb.vertices().collect();
        assert_eq!(vertices.len(), 4);
        assert!(
            vertices.iter().copied().any(|p| p == Point::new(3.0, 3.5)),
            "top left corner not is incorrect: {vertices:?}"
        );
        assert!(
            vertices.iter().copied().any(|p| p == Point::new(5.0, 3.5)),
            "top left corner not is incorrect: {vertices:?}"
        );
        assert!(
            vertices.iter().copied().any(|p| p == Point::new(5.0, 6.5)),
            "top left corner not is incorrect: {vertices:?}"
        );
        assert!(
            vertices.iter().copied().any(|p| p == Point::new(3.0, 6.5)),
            "top left corner not is incorrect: {vertices:?}"
        );
    }
}
