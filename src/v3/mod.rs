use math::{Point, Vector};
use shapes::Aabb;

trait ConvexPolygon {
    fn axes(&self) -> &[Vector];
}

trait Collides<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

mod math {
    pub(super) struct Point {
        x: f32,
        y: f32,
    }

    pub(super) struct Vector {
        x: f32,
        y: f32,
    }
}

mod shapes {
    use super::math::{Point, Vector};

    pub(super) struct Aabb {
        top_left: Point,
        bottom_right: Point,
    }
}
