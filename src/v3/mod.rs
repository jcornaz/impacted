use math::Vec2;
use sealed::sealed;
use shapes::Aabb;

#[sealed]
trait ConvexPolygon {
    fn axes(&self) -> &[Vec2];
}

#[sealed]
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
