mod point {
    use crate::v3::{math::Vec2, AxisProjection, Range};

    struct Point(Vec2);

    impl From<Vec2> for Point {
        fn from(value: Vec2) -> Self {
            Self(value)
        }
    }

    impl AxisProjection for Point {
        fn project(&self, axis: Vec2) -> crate::v3::Range {
            let p = self.0.dot(axis);
            Range::from_min_max(p, p)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use approx::assert_abs_diff_eq;
        use rstest::rstest;

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
            assert_abs_diff_eq!(point.into().project(axis), expected);
        }
    }
}

mod aabb {
    use crate::v3::{math::Vec2, AxisProjection, Range};

    struct Aabb {
        center: Vec2,
        half_size: Vec2,
    }

    impl Aabb {
        fn from_size(size: Vec2) -> Self {
            Self {
                center: Vec2::default(),
                half_size: size / 2.0,
            }
        }

        fn with_position(mut self, center: Vec2) -> Self {
            self.center = center;
            self
        }
    }

    impl AxisProjection for Aabb {
        fn project(&self, axis: Vec2) -> Range {
            let r1 = self.half_size.dot(axis).abs();
            let r2 = Vec2::new(-self.half_size.x, self.half_size.y)
                .dot(axis)
                .abs();
            let r = r1.max(r2);
            let shift = self.center.dot(axis);
            Range::from_min_max(shift - r, shift + r)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use approx::assert_abs_diff_eq;
        use rstest::rstest;

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
        #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(0.0, 0.0)), Vec2::new(1.0, 0.0), -1.5, 1.5)]
        #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(1.0, 0.0)), Vec2::new(1.0, 0.0), -0.5, 2.5)]
        #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(0.0, 1.0)), Vec2::new(1.0, 0.0), -1.5, 1.5)]
        #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(0.0, 1.0)), Vec2::new(0.0, 1.0), -1.0, 3.0)]
        fn test_axis_project(
            #[case] shape: Aabb,
            #[case] axis: Vec2,
            #[case] expected_min: f32,
            #[case] expected_max: f32,
        ) {
            let range = shape.project(axis);
            assert_abs_diff_eq!(range.min, expected_min);
            assert_abs_diff_eq!(range.max, expected_max);
        }
    }
}
