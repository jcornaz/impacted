use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

#[cfg(test)]
impl approx::AbsDiffEq for Vec2 {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon) && self.y.abs_diff_eq(&other.y, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_abs_diff_eq, assert_ulps_eq};
    use rstest::rstest;

    #[rstest]
    #[case([0.0, 0.0], [0.0, 0.0], [0.0, 0.0])]
    #[case([1.0, 2.0], [3.0, 4.0], [4.0, 6.0])]
    #[case([3.0, 4.0], [1.0, 2.0], [4.0, 6.0])]
    #[case([-1.0, 2.0], [3.0, 4.0], [2.0, 6.0])]
    #[case([-1.0, 2.0], [3.0, 5.5], [2.0, 7.5])]
    fn test_add(
        #[case] a: impl Into<Vec2>,
        #[case] b: impl Into<Vec2>,
        #[case] expected: impl Into<Vec2>,
    ) {
        let mut a = a.into();
        let b = b.into();
        let expected = expected.into();
        assert_abs_diff_eq!(a + b, expected);
        assert_abs_diff_eq!(b + a, expected);
        a += b;
        assert_abs_diff_eq!(a, expected);
    }

    #[rstest]
    #[case([0.0, 0.0], [0.0, 0.0], [0.0, 0.0])]
    #[case([1.0, 2.0], [3.0, 4.0], [-2.0, -2.0])]
    #[case([2.0, 7.5], [-1.0, 2.0], [3.0, 5.5])]
    #[case([2.0, 7.5], [3.0, 5.5], [-1.0, 2.0])]
    fn test_sub(
        #[case] a: impl Into<Vec2>,
        #[case] b: impl Into<Vec2>,
        #[case] expected: impl Into<Vec2>,
    ) {
        let mut a = a.into();
        let b = b.into();
        let expected = expected.into();
        assert_abs_diff_eq!(a - b, expected);
        a -= b;
        assert_abs_diff_eq!(a, expected);
    }

    #[rstest]
    #[case([0.0, 0.0], [0.0, 0.0], 0.0)]
    #[case([1.0, 0.0], [2.0, 0.0], 2.0)]
    #[case([2.0, 0.0], [3.0, 0.0], 6.0)]
    #[case([2.0, 1.0], [3.0, 1.0], 7.0)]
    #[case([2.0, 3.0], [3.0, 4.0], 18.0)]
    #[case(Vec2::new(2.0, 3.0), Vec2::new(3.0, 4.0), 18.0)]
    #[case([2.0, 3.0], [-3.0, 4.0], 6.0)]
    fn test_dot(#[case] v1: impl Into<Vec2>, #[case] v2: impl Into<Vec2>, #[case] expected: f32) {
        assert_ulps_eq!(v1.into().dot(v2.into()), expected);
    }
}
