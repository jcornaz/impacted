//! Basic convex shape types for collision test
//!
//! The shape types of this module do not include their position/orientation.
//! To test collision between shapes positioned in the world,
//! they need to be combined with the [`transform`](crate::transform) module.
//!
//! It is possible to extend this collection of types by implementing the [`Support`] trait

use crate::{glam::Vec2, Support};

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

impl Support for Circle {
    /// Returns the farthest point of the shape in the given direction
    ///
    /// # Example
    ///
    /// ```
    /// # use impacted::{Support, shapes::Circle};
    /// # use glam::Vec2;
    /// assert_eq!(Circle::new(2.0).support(Vec2::X), Vec2::X * 2.0);
    /// assert_eq!(Circle::new(3.0).support(Vec2::Y), Vec2::Y * 3.0);
    /// assert_eq!(Circle::new(0.0).support(Vec2::X), Vec2::ZERO);
    /// ```
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
    /// Creates a rectangle from its size
    #[inline]
    #[must_use]
    pub fn from_size(size: Vec2) -> Self {
        Self::from_half_extents(size * 0.5)
    }

    /// Creates a rectangle from its half-extents
    #[inline]
    #[must_use]
    pub fn from_half_extents(half_extents: Vec2) -> Self {
        Self { half_extents }
    }
}

impl Support for Rectangle {
    /// Returns the farthest point of the shape in the given direction
    ///
    /// # Example
    ///
    /// ```
    /// # use impacted::{Support, shapes::Rectangle};
    /// # use glam::Vec2;  
    /// let  rectangle = Rectangle::from_half_extents(Vec2::new(3.0, 2.0));
    /// assert_eq!(rectangle.support(Vec2::new(1., 1.)), Vec2::new(3., 2.));
    /// assert_eq!(rectangle.support(Vec2::new(-1., 1.)), Vec2::new(-3., 2.));
    /// assert_eq!(rectangle.support(Vec2::new(1., -1.)), Vec2::new(3., -2.));
    /// assert_eq!(rectangle.support(Vec2::new(-1., -1.)), Vec2::new(-3., -2.));
    /// ```
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
