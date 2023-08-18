use crate::v3::{Point, Vec2};
use glam;

impl From<glam::Vec2> for Vec2 {
    fn from(v: glam::Vec2) -> Self {
        Vec2::new(v.x, v.y)
    }
}

impl From<Vec2> for glam::Vec2 {
    fn from(v: Vec2) -> Self {
        glam::Vec2::new(v.x, v.y)
    }
}

impl From<glam::Vec2> for Point {
    fn from(v: glam::Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Point> for glam::Vec2 {
    fn from(p: Point) -> Self {
        glam::Vec2::new(p.x(), p.y())
    }
}
