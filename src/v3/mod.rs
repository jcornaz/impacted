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

trait ConvexPolygon {
    fn axes(&self) -> &[Vec2];
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
