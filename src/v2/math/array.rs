use core::ops::{Add, Mul};

use super::Dot;

impl<S: Copy + Add<S, Output = S> + Mul<S, Output = S>> Dot for [S; 2] {
    type Output = S;
    fn dot(self, other: Self) -> Self::Output {
        (self[0] * other[0]) + (self[1] * other[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use rstest::rstest;

    #[rstest]
    #[case([0.0, 0.0], [0.0, 0.0], 0.0)]
    #[case([1.0, 0.0], [2.0, 0.0], 2.0)]
    #[case([2.0, 0.0], [3.0, 0.0], 6.0)]
    #[case([2.0, 1.0], [3.0, 1.0], 7.0)]
    #[case([2.0, 3.0], [3.0, 4.0], 18.0)]
    #[case([2.0, 3.0], [-3.0, 4.0], 6.0)]
    fn test_dot(#[case] v1: [f32; 2], #[case] v2: [f32; 2], #[case] expected: f32) {
        assert_ulps_eq!(v1.dot(v2), expected);
    }
}
