pub use aabb::Aabb;
pub use point::Point;

mod point {
    use core::{
        iter,
        ops::{Add, AddAssign, Sub, SubAssign},
    };

    use sealed::sealed;

    use crate::v3::{__seal_shape, math::Vec2, Range, Shape};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Point(Vec2);

    impl Point {
        pub const ORIGIN: Self = Self(Vec2::ZERO);

        #[must_use]
        pub fn new(x: f32, y: f32) -> Self {
            Self(Vec2::new(x, y))
        }

        #[must_use]
        pub fn x(self) -> f32 {
            self.0.x
        }

        #[must_use]
        pub fn y(self) -> f32 {
            self.0.y
        }
    }

    impl From<Vec2> for Point {
        fn from(value: Vec2) -> Self {
            Self(value)
        }
    }

    impl From<Point> for Vec2 {
        fn from(Point(v): Point) -> Self {
            v
        }
    }

    #[sealed]
    impl Shape for Point {
        type AxisIter = iter::Empty<Vec2>;
        type FocalsIter = iter::Once<Point>;
        type VerticesIter = iter::Empty<Point>;

        fn axes(&self) -> Self::AxisIter {
            iter::empty()
        }

        fn focals(&self) -> Self::FocalsIter {
            iter::once(*self)
        }

        fn vertices(&self) -> Self::VerticesIter {
            iter::empty()
        }

        fn project_on(&self, axis: Vec2) -> Range {
            let p = self.0.dot(axis);
            Range::from_min_max(p, p)
        }
    }

    impl AddAssign<Vec2> for Point {
        fn add_assign(&mut self, rhs: Vec2) {
            self.0 += rhs;
        }
    }

    impl Add<Vec2> for Point {
        type Output = Self;
        fn add(mut self, rhs: Vec2) -> Self::Output {
            self += rhs;
            self
        }
    }

    impl SubAssign<Vec2> for Point {
        fn sub_assign(&mut self, rhs: Vec2) {
            self.0 -= rhs;
        }
    }

    impl Sub<Vec2> for Point {
        type Output = Self;
        fn sub(mut self, rhs: Vec2) -> Self::Output {
            self -= rhs;
            self
        }
    }

    #[cfg(test)]
    impl approx::AbsDiffEq for Point {
        type Epsilon = f32;
        fn default_epsilon() -> Self::Epsilon {
            f32::default_epsilon()
        }
        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.0.abs_diff_eq(&other.0, epsilon)
        }
    }

    #[cfg(test)]
    mod tests {
        use approx::assert_abs_diff_eq;
        use rstest::rstest;

        use super::*;

        #[rstest]
        #[case(Vec2::ZERO, Vec2::ZERO, 0.0)]
        #[case(Vec2::ZERO, Vec2::X, 0.0)]
        #[case(Vec2::X, Vec2::X, 1.0)]
        #[case(Vec2::X, Vec2::Y, 0.0)]
        #[case(Vec2::Y, Vec2::Y, 1.0)]
        #[case(Vec2::Y, Vec2::X, 0.0)]
        #[case(Vec2::new(3.0, 4.0), Vec2::X, 3.0)]
        #[case(Vec2::new(3.0, 4.0), Vec2::Y, 4.0)]
        #[case(Vec2::new(3.0, 4.0),Vec2::new(3.0/5.0, 4.0/5.0),5.0)]
        #[case(Vec2::new(3.0, 3.0),Vec2::new(2f32.sqrt(), -(2f32.sqrt())),0.0)]
        fn test_axis_projection(
            #[case] point: impl Into<Point>,
            #[case] axis: Vec2,
            #[case] expected: f32,
        ) {
            let expected = Range::from_min_max(expected, expected);
            assert_abs_diff_eq!(point.into().project_on(axis), expected);
        }

        #[test]
        fn test_sat_axes() {
            assert_eq!(Point::from(Vec2::ZERO).axes().next(), None);
        }
    }
}

mod aabb {
    use core::{array, iter};
    use sealed::sealed;

    use crate::v3::{__seal_shape, math::Vec2, Point, Range, Shape};

    pub struct Aabb {
        center: Point,
        half_size: Vec2,
    }

    impl Aabb {
        #[must_use]
        pub fn from_size(size: Vec2) -> Self {
            Self {
                center: Point::ORIGIN,
                half_size: size / 2.0,
            }
        }

        #[must_use]
        pub fn with_center_at(mut self, center: Point) -> Self {
            self.center = center;
            self
        }
    }

    #[sealed]
    impl Shape for Aabb {
        type AxisIter = array::IntoIter<Vec2, 2>;
        type FocalsIter = iter::Empty<Point>;
        type VerticesIter = array::IntoIter<Point, 4>;

        fn axes(&self) -> Self::AxisIter {
            [Vec2::X, Vec2::Y].into_iter()
        }

        fn focals(&self) -> Self::FocalsIter {
            iter::empty()
        }

        fn vertices(&self) -> Self::VerticesIter {
            [
                (self.center - self.half_size),
                (self.center + Vec2::new(self.half_size.x, -self.half_size.y)),
                (self.center + self.half_size),
                (self.center + Vec2::new(-self.half_size.x, self.half_size.y)),
            ]
            .into_iter()
        }

        fn project_on(&self, axis: Vec2) -> Range {
            let r1 = self.half_size.dot(axis).abs();
            let r2 = Vec2::new(-self.half_size.x, self.half_size.y)
                .dot(axis)
                .abs();
            let r = r1.max(r2);
            let shift = Vec2::from(self.center).dot(axis);
            Range::from_min_max(shift - r, shift + r)
        }
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
}
