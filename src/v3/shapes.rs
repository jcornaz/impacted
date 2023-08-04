use super::{math::Vec2, AxisProjection, Range};

pub(super) struct Aabb {
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
    use core::ops::RangeInclusive;

    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        Aabb::from_size(Vec2::new(0.0, 0.0)),
        Vec2::new(1.0, 0.0),
        Range::from_min_max(0.0, 0.0)
    )]
    #[case(Aabb::from_size(Vec2::new(2.0, 0.0)), Vec2::new(1.0, 0.0), Range::from_min_max(-1.0, 1.0))]
    #[case(Aabb::from_size(Vec2::new(2.0, 0.0)), Vec2::new(-1.0, 0.0), Range::from_min_max(-1.0, 1.0))]
    #[case(Aabb::from_size(Vec2::new(0.0, 2.0)), Vec2::new(0.0, 1.0), Range::from_min_max(-1.0, 1.0))]
    #[case(Aabb::from_size(Vec2::new(0.0, 2.0)), Vec2::new(0.0, -1.0), Range::from_min_max(-1.0, 1.0))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(1.0, 0.0), Range::from_min_max(-1.5, 1.5))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(0.0, 1.0), Range::from_min_max(-2.0, 2.0))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(1.5, 2.0), Range::from_min_max(-6.25, 6.25))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(1.5, -2.0), Range::from_min_max(-6.25, 6.25))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)), Vec2::new(-1.5, 2.0), Range::from_min_max(-6.25, 6.25))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(0.0, 0.0)), Vec2::new(1.0, 0.0), Range::from_min_max(-1.5, 1.5))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(1.0, 0.0)), Vec2::new(1.0, 0.0), Range::from_min_max(-0.5, 2.5))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(0.0, 1.0)), Vec2::new(1.0, 0.0), Range::from_min_max(-1.5, 1.5))]
    #[case(Aabb::from_size(Vec2::new(3.0, 4.0)).with_position(Vec2::new(0.0, 1.0)), Vec2::new(0.0, 1.0), Range::from_min_max(-1.0, 3.0))]
    fn test_axis_project(#[case] shape: Aabb, #[case] axis: Vec2, #[case] expected: Range) {
        let range = shape.project(axis);
        assert_abs_diff_eq!(range, expected);
    }
}
