#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs, clippy::pedantic)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

//! 2d collision test for arbitrary convex shapes
//!
//! # Usage example
//!
//! ```
//! # use core::convert::TryInto;
//! use impacted::prelude::*;
//! use glam::{Vec2, Affine2};
//! # fn main() -> Result<(), impacted::transform::Error> {
//!
//! // Create a circle
//! let circle = shapes::Circle::new(1.0);
//! let circle_transform = Transform::default();
//!
//! // Create a rectangle
//! let rect = shapes::Rectangle::from_half_extents(Vec2::splat(2.0));
//! let rect_transform1: Transform = Affine2::from_translation(Vec2::new(2.0, 0.0)).try_into()?;
//! let rect_transform2: Transform = Affine2::from_translation(Vec2::new(0.0, 4.0)).try_into()?;
//!
//! // Then we can test for collision
//! assert!(impacted::are_collided(
//!     &TransformedShape::new(&circle_transform, &circle),
//!     &TransformedShape::new(&rect_transform1, &rect)
//! ));
//! assert!(!impacted::are_collided(
//!     &TransformedShape::new(&circle_transform, &circle),
//!     &TransformedShape::new(&rect_transform2, &rect)
//! ));
//! # Ok(()) }
//! ```

/// Re-export of glam types
pub use glam;

use self::{glam::Vec2, simplex::Simplex};

mod minkowski;
pub mod shapes;
mod simplex;
pub mod transform;

/// Re-export of most commonly used types
pub mod prelude {
    pub use crate::{
        shapes,
        transform::{Transform, TransformedShape},
        Support,
    };
}

/// Trait to be implemented by shape types to support collision detection
///
/// The shape implementing this trait **must be convex**.
///
/// # Examples of implementation
///
/// ## Circle
///
/// ```
/// # use impacted::{Support, glam::Vec2};
/// struct Circle { center: Vec2, radius: f32 };
/// impl Support for Circle {
///     /// Returns the farthest point of the shape in the given direction
///     fn support(&self, direction: Vec2) -> Vec2 {
///         // For a circle, the farthest point, is equal to `direction` with the length clamped at radius length
///         self.center + direction.clamp_length(self.radius, self.radius)
///     }
/// }
/// ```
///
/// ## Rectangle
///
/// ```
/// # use impacted::{Support, glam::Vec2};
/// struct Rectangle { center: Vec2, extents: Vec2 };
/// impl Support for Rectangle {
///     /// Returns the farthest point of the shape in the given direction
///     fn support(&self, direction: Vec2) -> Vec2 {
///         // For a rectangle, we return the farthest vertex.
///         // In case multiple points are equally far in the direction, we can return any.
///         self.center + Vec2::new(
///             if direction.x >= 0.0 { self.extents.x } else { -self.extents.x },
///             if direction.y >= 0.0 { self.extents.y } else { -self.extents.y },
///         )
///     }
/// }
/// ```
pub trait Support {
    /// Returns the farthest point of the shape in the given direction
    fn support(&self, direction: Vec2) -> Vec2;
}

/// Returns true if the two convex shapes geometries are overlapping
#[inline]
#[must_use]
pub fn are_collided(shape1: &impl Support, shape2: &impl Support) -> bool {
    let difference = minkowski::Difference { shape1, shape2 };
    find_simplex_enclosing_origin(&difference, Vec2::X).is_some()
}

fn find_simplex_enclosing_origin(shape: &impl Support, initial_direction: Vec2) -> Option<Simplex> {
    let mut simplex = {
        let first_point = shape.support(initial_direction);
        if first_point.dot(initial_direction) <= 0.0 {
            return None;
        }
        Simplex::new(first_point)
    };

    while let Some(direction) = simplex.next() {
        let point = shape.support(direction);
        if point.dot(direction) <= 0.0 {
            return None;
        }
        simplex.insert(point);
    }
    Some(simplex)
}
