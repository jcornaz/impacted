#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs, clippy::pedantic)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

//! 2d collision test
//!
//! The central type is [`CollisionShape`]. Once a collision shape is created and positioned (with a [`Transform`])
//! is is possible to call [`CollisionShape::is_collided_with`] to test for collision with another shape.
//!
//! # Example
//!
//! ```
//! use impacted::{CollisionShape, Transform};
//!
//! // The examples of this crate use glam.
//! // But you may use another math library instead.
//! use glam::Vec2;
//!
//! // Create a circle
//! let circle = CollisionShape::new_circle(1.0);
//!
//! // Create a rectangle
//! let mut rect1 = CollisionShape::new_rectangle(4.0, 4.0)
//!     .with_transform(Transform::from_translation(Vec2::new(2.0, 0.0)));
//!
//! // Create another rectangle
//! let mut rect2 = rect1.clone()
//!     .with_transform(Transform::from_translation(Vec2::new(0.0, 4.0)));
//!
//! // Then we can test for collision
//! assert!(circle.is_collided_with(&rect1));
//! assert!(!circle.is_collided_with(&rect2));
//! ```

use glam::Vec2;

use crate::shapes::ShapeData;
pub use crate::transform::Transform;

use self::simplex::Simplex;

mod gjk;
mod interop;
mod minkowski;
pub mod shapes;
mod simplex;
mod transform;

/// Error returned on invalid input
#[non_exhaustive]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    /// Error returned when trying to create a [`Transform`] from a non-invertible matrix
    #[cfg_attr(feature = "std", error("non-invertible transform"))]
    NonInvertibleTransform,
}

/// A collision shape
///
/// This is the entry point for collision detection.
///
/// See [crate](crate) level documentation for more info and examples.
#[derive(Debug, Clone)]
pub struct CollisionShape {
    transform: Transform,
    data: ShapeData,
}

impl<S: Into<ShapeData>> From<S> for CollisionShape {
    fn from(shape: S) -> Self {
        Self {
            transform: Transform::default(),
            data: shape.into(),
        }
    }
}

impl CollisionShape {
    /// Create a circle from its radius
    ///
    /// The origin is in the center of the circle
    #[inline]
    #[must_use]
    pub fn new_circle(radius: f32) -> Self {
        shapes::Circle::new(radius).into()
    }

    /// Create a rectangle from its width and height
    ///
    /// The origin is in the center of the rectangle
    #[inline]
    #[must_use]
    pub fn new_rectangle(width: f32, height: f32) -> Self {
        shapes::Rectangle::new(width, height).into()
    }

    /// Set the transform (translation, rotation and scale)
    ///
    /// This is equivalent to [`set_transform`](Self::set_transform), but in a builder style,
    /// useful to set the transform directly at creation
    #[inline]
    #[must_use]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.set_transform(transform);
        self
    }

    /// Set the transform (translation, rotation and scale)
    #[inline]
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    /// Returns true if the two convex shapes geometries are overlapping
    #[inline]
    #[must_use]
    pub fn is_collided_with(&self, other: &Self) -> bool {
        let difference = minkowski::Difference {
            shape1: self,
            shape2: other,
        };
        let initial_axis = other.transform.position() - self.transform.position();
        gjk::find_simplex_enclosing_origin(&difference, initial_axis).is_some()
    }
}

trait Support {
    /// Returns the farthest point of the shape in the given direction
    fn support(&self, direction: Vec2) -> Vec2;
}
