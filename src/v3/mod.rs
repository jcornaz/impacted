use math::Vector;
use shapes::Aabb;

trait ConvexPolygon {
    fn axes(&self) -> &[Vector];
}

trait Collides<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

mod math;

mod shapes {
    use super::math::Vector;

    pub(super) struct Aabb {
        top_left: Vector,
        bottom_right: Vector,
    }
}
