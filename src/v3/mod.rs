use math::Vec2;
use shapes::Aabb;

trait ConvexPolygon {
    fn axes(&self) -> &[Vec2];
}

trait Collides<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

mod math;

mod shapes {
    use super::math::Vec2;

    pub(super) struct Aabb {
        top_left: Vec2,
        bottom_right: Vec2,
    }
}
