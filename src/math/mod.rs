mod array;
mod glam_0_24;

pub(crate) trait Dot {
    type Scalar;
    fn dot(self, other: Self) -> Self::Scalar;
}

pub(crate) trait CmpToZero: Copy {
    fn is_negative(self) -> bool;
    fn is_zero(self) -> bool;
    fn is_positive(self) -> bool;
}

pub(crate) trait Perp {
    fn perp(self) -> Self;
}

pub(crate) trait MagnitudeSquared {
    type Scalar;
    fn magnitude_squared(self) -> Self::Scalar;
}

impl<V: Dot + Copy> MagnitudeSquared for V {
    type Scalar = <V as Dot>::Scalar;
    fn magnitude_squared(self) -> Self::Scalar {
        self.dot(self)
    }
}

impl CmpToZero for f32 {
    fn is_negative(self) -> bool {
        self < 0.0
    }

    fn is_zero(self) -> bool {
        self == 0.0
    }

    fn is_positive(self) -> bool {
        self > 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use rstest::rstest;

    #[rstest]
    #[case([0.0, 0.0], 0.0)]
    #[case([2.0, 3.0], 13.0)]
    #[case([3.0, 4.0], 25.0)]
    #[case([4.0, 3.0], 25.0)]
    fn test_magnitude_squared(#[case] vector: [f32; 2], #[case] expected: f32) {
        assert_ulps_eq!(vector.magnitude_squared(), expected);
    }
}