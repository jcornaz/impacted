//! Collection of shape data that can be used to create a [`CollisionShape`](crate::CollisionShape)

use glam::Vec2;

use crate::Support;

/// Geometric information about a shape
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ShapeData {
    /// A circle
    ///
    /// See [`Circle`]
    Circle(Circle),
    /// A circle
    ///
    /// See [`Rectangle`]
    Rectangle(Rectangle),
}

impl Support for ShapeData {
    fn support(&self, direction: Vec2) -> Vec2 {
        match self {
            ShapeData::Circle(circle) => circle.support(direction),
            ShapeData::Rectangle(rect) => rect.support(direction),
        }
    }
}

/// A circle
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    radius: f32,
}

impl Circle {
    /// Create a circle from its radius
    #[inline]
    #[must_use]
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl From<Circle> for ShapeData {
    #[inline]
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

impl Support for Circle {
    fn support(&self, direction: Vec2) -> Vec2 {
        direction.clamp_length(self.radius, self.radius)
    }
}

/// A rectangle
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    half_extents: Vec2,
}

impl Rectangle {
    /// Creates a rectangle from its width and height
    ///
    /// The origin is in the center of the rectangle
    #[inline]
    #[must_use]
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_extents: Vec2::new(width * 0.5, height * 0.5).abs(),
        }
    }
}

impl From<Rectangle> for ShapeData {
    #[inline]
    fn from(rect: Rectangle) -> Self {
        Self::Rectangle(rect)
    }
}

impl Support for Rectangle {
    fn support(&self, direction: Vec2) -> Vec2 {
        let mut support = self.half_extents;
        if direction.x < 0.0 {
            support.x = -support.x;
        }
        if direction.y < 0.0 {
            support.y = -support.y;
        }
        support
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        assert_eq!(Circle::new(2.0).support(Vec2::X), Vec2::X * 2.0);
        assert_eq!(Circle::new(3.0).support(Vec2::Y), Vec2::Y * 3.0);
        assert_eq!(Circle::new(0.0).support(Vec2::X), Vec2::ZERO);
    }

    #[test]
    fn rectangle() {
        let rectangle = Rectangle::new(6.0, 4.0);
        assert_eq!(rectangle.support(Vec2::new(1., 1.)), Vec2::new(3., 2.));
        assert_eq!(rectangle.support(Vec2::new(-1., 1.)), Vec2::new(-3., 2.));
        assert_eq!(rectangle.support(Vec2::new(1., -1.)), Vec2::new(3., -2.));
        assert_eq!(rectangle.support(Vec2::new(-1., -1.)), Vec2::new(-3., -2.));
    }
}
