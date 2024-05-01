use core::{
    iter,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use sealed::sealed;

use super::{vector::Vec2, Range, Shape, __seal_shape};

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

impl From<[f32; 2]> for Point {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Point> for [f32; 2] {
    fn from(Point(v): Point) -> Self {
        v.into()
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Point::new(x, y)
    }
}

impl From<Point> for (f32, f32) {
    fn from(Point(v): Point) -> Self {
        v.into()
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
    fn axes(&self) -> impl Iterator<Item = Vec2> {
        iter::empty()
    }

    fn focals(&self) -> impl Iterator<Item = Point> {
        iter::empty()
    }

    fn vertices(&self) -> impl Iterator<Item = Point> {
        iter::once(*self)
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
