use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub(super) fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub(super) fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
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

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
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
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne, assert_ulps_eq};
    use rstest::rstest;

    #[rstest]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), 0.0)]
    #[case(Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0), 1.0)]
    #[case(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), 0.0)]
    #[case(Vec2::new(0.0, 1.0), Vec2::new(0.0, 1.0), 1.0)]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), 0.0)]
    #[case(Vec2::new(2.0, 3.0), Vec2::new(5.0, 7.0), 31.0)]
    #[case(Vec2::new(5.0, 7.0), Vec2::new(2.0, 3.0), 31.0)]
    fn test_dot(#[case] v1: Vec2, #[case] v2: Vec2, #[case] expected: f32) {
        assert_abs_diff_eq!(v1.dot(v2), expected);
    }

    #[rstest]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0))]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0))]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, 1.0))]
    #[case(Vec2::new(0.0, 1.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0))]
    #[case(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0), Vec2::new(4.0, 6.0))]
    #[case(Vec2::new(-1.0, 2.0), Vec2::new(3.0, -4.0), Vec2::new(2.0, -2.0))]
    fn test_add(#[case] v1: Vec2, #[case] v2: Vec2, #[case] expected: Vec2) {
        assert_abs_diff_eq!(v1 + v2, expected);
        let mut sum = v1;
        sum += v2;
        assert_abs_diff_eq!(sum, expected);
    }

    #[rstest]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0))]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(-1.0, 0.0))]
    #[case(Vec2::new(0.0, 1.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0))]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0), Vec2::new(0.0, -1.0))]
    #[case(Vec2::new(10.0, 11.0), Vec2::new(3.0, 5.0), Vec2::new(7.0, 6.0))]
    #[case(Vec2::new(3.0, 5.0), Vec2::new(10.0, 11.0), Vec2::new(-7.0, -6.0))]
    fn test_sub(#[case] v1: Vec2, #[case] v2: Vec2, #[case] expected: Vec2) {
        assert_abs_diff_eq!(v1 - v2, expected);
        let mut res = v1;
        res -= v2;
        assert_abs_diff_eq!(res, expected);
    }

    #[rstest]
    #[case(Vec2::new(0.0, 0.0), 1.0, Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(1.0, 0.0), 2.0, Vec2::new(2.0, 0.0))]
    #[case(Vec2::new(0.0, 1.0), 2.0, Vec2::new(0.0, 2.0))]
    #[case(Vec2::new(2.0, 3.0), 4.0, Vec2::new(8.0, 12.0))]
    fn test_mul_div(#[case] v: Vec2, #[case] f: f32, #[case] expected: Vec2) {
        assert_abs_diff_eq!(v * f, expected);
        assert_abs_diff_eq!(v / f.recip(), expected);
        let mut res = v;
        res *= f;
        assert_abs_diff_eq!(res, expected);
        res = v;
        res /= f.recip();
        assert_abs_diff_eq!(res, expected);
    }
}
