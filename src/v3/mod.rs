mod math;
mod range;
mod shapes;

use math::Vec2;
use range::Range;

trait SatShape: AxisProjection {
    type AxisIter: Iterator<Item = Vec2>;
    fn axes(&self) -> Self::AxisIter;
}

trait AxisProjection {
    fn project(&self, axis: Vec2) -> Range;
}
