use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec2 {
    pub(super) x: f32,
    pub(super) y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);

    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn splat(value: f32) -> Self {
        Self::new(value, value)
    }

    #[must_use]
    pub fn x(self) -> f32 {
        self.x
    }

    #[must_use]
    pub fn y(self) -> f32 {
        self.y
    }

    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn normalize(self) -> Option<Self> {
        let normal = self / self.magnitude();
        if !normal.x.is_finite() {
            return None;
        }
        Some(normal)
    }

    #[must_use]
    #[cfg(test)]
    pub(super) fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(Vec2 { x, y }: Vec2) -> Self {
        [x, y]
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Vec2::new(x, y)
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(Vec2 { x, y }: Vec2) -> Self {
        (x, y)
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

impl Neg for Vec2 {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
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
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Vec2::ZERO, Vec2::ZERO, 0.0)]
    #[case(Vec2::X, Vec2::X, 1.0)]
    #[case(Vec2::X, Vec2::ZERO, 0.0)]
    #[case(Vec2::Y, Vec2::Y, 1.0)]
    #[case(Vec2::ZERO, Vec2::Y, 0.0)]
    #[case(Vec2::new(2.0, 3.0), Vec2::new(5.0, 7.0), 31.0)]
    #[case(Vec2::new(5.0, 7.0), Vec2::new(2.0, 3.0), 31.0)]
    fn test_dot(#[case] v1: Vec2, #[case] v2: Vec2, #[case] expected: f32) {
        assert_abs_diff_eq!(v1.dot(v2), expected);
    }

    #[rstest]
    #[case(Vec2::ZERO, Vec2::ZERO, Vec2::ZERO)]
    #[case(Vec2::X, Vec2::ZERO, Vec2::X)]
    #[case(Vec2::ZERO, Vec2::X, Vec2::X)]
    #[case(Vec2::ZERO, Vec2::Y, Vec2::Y)]
    #[case(Vec2::Y, Vec2::ZERO, Vec2::Y)]
    #[case(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0), Vec2::new(4.0, 6.0))]
    #[case(Vec2::new(-1.0, 2.0), Vec2::new(3.0, -4.0), Vec2::new(2.0, -2.0))]
    fn test_add(#[case] v1: Vec2, #[case] v2: Vec2, #[case] expected: Vec2) {
        assert_abs_diff_eq!(v1 + v2, expected);
        let mut sum = v1;
        sum += v2;
        assert_abs_diff_eq!(sum, expected);
    }

    #[rstest]
    #[case(Vec2::ZERO, Vec2::ZERO, Vec2::ZERO)]
    #[case(Vec2::X, Vec2::ZERO, Vec2::X)]
    #[case(Vec2::ZERO, Vec2::X, Vec2::new(-1.0, 0.0))]
    #[case(Vec2::Y, Vec2::ZERO, Vec2::Y)]
    #[case(Vec2::ZERO, Vec2::Y, Vec2::new(0.0, -1.0))]
    #[case(Vec2::new(10.0, 11.0), Vec2::new(3.0, 5.0), Vec2::new(7.0, 6.0))]
    #[case(Vec2::new(3.0, 5.0), Vec2::new(10.0, 11.0), Vec2::new(-7.0, -6.0))]
    fn test_sub(#[case] v1: Vec2, #[case] v2: Vec2, #[case] expected: Vec2) {
        assert_abs_diff_eq!(v1 - v2, expected);
        let mut res = v1;
        res -= v2;
        assert_abs_diff_eq!(res, expected);
    }

    #[rstest]
    #[case(Vec2::ZERO, 1.0, Vec2::ZERO)]
    #[case(Vec2::X, 2.0, Vec2::new(2.0, 0.0))]
    #[case(Vec2::Y, 2.0, Vec2::new(0.0, 2.0))]
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

    #[rstest]
    #[case(Vec2::ZERO, Vec2::ZERO)]
    #[case(Vec2::X, Vec2::new(-1.0, 0.0))]
    #[case(Vec2::Y, Vec2::new(0.0, -1.0))]
    #[case(Vec2::new(-2.3, 4.5), Vec2::new(2.3, -4.5))]
    fn test_negate(#[case] v: Vec2, #[case] expected: Vec2) {
        assert_abs_diff_eq!(-v, expected);
    }

    #[rstest]
    #[case(Vec2::ZERO, 0.0)]
    #[case(Vec2::X, 1.0)]
    #[case(-Vec2::X, 1.0)]
    #[case(Vec2::Y, 1.0)]
    #[case(-Vec2::Y, 1.0)]
    #[case(Vec2::new(3.0, 4.0), 5.0)]
    fn test_magnitude(#[case] vector: Vec2, #[case] expected_magnitude: f32) {
        assert_abs_diff_eq!(vector.magnitude(), expected_magnitude);
        assert_abs_diff_eq!(
            vector.magnitude_squared(),
            expected_magnitude * expected_magnitude
        );
    }

    #[rstest]
    fn normalize_zero_returns_none() {
        assert_eq!(Vec2::ZERO.normalize(), None);
    }

    #[rstest]
    #[case(Vec2::X, Vec2::X)]
    #[case(-Vec2::X, -Vec2::X)]
    #[case(Vec2::Y, Vec2::Y)]
    #[case(Vec2::X * 2.0, Vec2::X)]
    #[case(Vec2::Y * 2.0, Vec2::Y)]
    #[case(Vec2::new(3.0, 4.0), Vec2::new(0.6, 0.8))]
    fn normalize_returns_expected(#[case] vector: Vec2, #[case] expected: Vec2) {
        assert_abs_diff_eq!(vector.normalize().unwrap(), expected);
    }

    #[rstest]
    #[case(Vec2::X, Vec2::Y)]
    #[case(Vec2::Y, -Vec2::X)]
    #[case(-Vec2::X, -Vec2::Y)]
    #[case(-Vec2::Y, Vec2::X)]
    #[case(Vec2::new(1.0, 2.0), Vec2::new(-2.0, 1.0))]
    #[case(Vec2::new(-3.0, -4.0), Vec2::new(4.0, -3.0))]
    fn test_perp(#[case] vector: Vec2, #[case] expected: Vec2) {
        assert_eq!(vector.perp(), expected);
    }
}
