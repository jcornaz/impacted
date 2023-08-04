#[derive(Debug, Copy, Clone)]
pub(super) struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl From<[f32; 2]> for Vector {
    fn from([x, y]: [f32; 2]) -> Self {
        Self::new(x, y)
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
    #[case(Vector::new(2.0, 3.0), Vector::new(3.0, 4.0), 18.0)]
    #[case([2.0, 3.0], [-3.0, 4.0], 6.0)]
    fn test_dot(
        #[case] v1: impl Into<Vector>,
        #[case] v2: impl Into<Vector>,
        #[case] expected: f32,
    ) {
        assert_ulps_eq!(v1.into().dot(v2.into()), expected);
    }
}
