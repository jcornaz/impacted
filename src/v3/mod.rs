mod math;
mod shapes;

use math::Vec2;
use sealed::sealed;

#[sealed]
trait Collides<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

#[sealed]
trait Cast<Rhs> {
    fn cast(&self, other: &Rhs) -> Option<Vec2>;
}

trait SatAxes {
    type Iter: Iterator<Item = Vec2>;
    fn axes(&self) -> Self::Iter;
}

trait AxisProjection {
    fn project(&self, axis: Vec2) -> Range;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    min: f32,
    max: f32,
}

impl Range {
    fn from_min_max(min: f32, max: f32) -> Self {
        debug_assert!(min <= max);
        Self { min, max }
    }
}

#[sealed]
impl Collides<Range> for Range {
    fn collides(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

#[cfg(test)]
impl approx::AbsDiffEq for Range {
    type Epsilon = f32;
    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.min.abs_diff_eq(&other.min, epsilon) && self.max.abs_diff_eq(&other.max, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((0.0, 1.0), (0.5, 1.5))]
    #[case((0.0, 1.0), (1.0, 1.5))]
    #[case((0.0, 1.0), (-1.0, 0.5))]
    #[case((0.0, 1.0), (-1.0, 0.0))]
    #[case((0.0, 1.0), (0.1, 0.9))]
    #[case((0.0, 1.0), (-1.0, 2.0))]
    #[case((0.0, 0.0), (0.0, 0.0))]
    fn range_should_overlap(#[case] r1: (f32, f32), #[case] r2: (f32, f32)) {
        let r1 = Range::from_min_max(r1.0, r1.1);
        let r2 = Range::from_min_max(r2.0, r2.1);
        assert!(r1.collides(&r2), "{r1:?} does not overlap {r2:?}");
    }

    #[rstest]
    #[case((0.0, 1.0), (1.1, 1.5))]
    #[case((2.0, 3.0), (0.0, 1.0))]
    #[case((2.0, 3.0), (0.0, 1.0))]
    fn range_should_not_overlap(#[case] r1: (f32, f32), #[case] r2: (f32, f32)) {
        let r1 = Range::from_min_max(r1.0, r1.1);
        let r2 = Range::from_min_max(r2.0, r2.1);
        assert!(!r1.collides(&r2), "{r1:?} overlaps {r2:?}");
    }
}
