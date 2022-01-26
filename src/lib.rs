#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs, clippy::pedantic)]
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

//! 2d collision test for arbitrary convex shapes
//!
//! # Usage example
//!
//! ```
//! use gjk2d::{math::Vec2, Support, collides};
//!
//! // We need shape types
//! struct Circle { center: Vec2, radius: f32 };
//! impl Support for Circle {
//!     fn support(&self, direction: Vec2) -> Vec2 {
//!         // We have to return the farthest point of the shape in the given direction
//!         self.center + direction.clamp_length(self.radius, self.radius)
//!     }
//! }
//!
//! // Then we can test for collision
//! assert!(collides(
//!     &Circle { center: Vec2::new(2.0, 3.0), radius: 1.0 },
//!     &Circle { center: Vec2::new(4.0, 3.0), radius: 2.0 },
//! ));
//! assert!(!collides(
//!     &Circle { center: Vec2::new(2.0, 3.0), radius: 1.0 },
//!     &Circle { center: Vec2::new(2.0, 7.0), radius: 2.0 },
//! ));
//! ```

use self::{math::Vec2, simplex::Simplex};

/// Re-export of [glam](https://docs.rs/glam/0.20.2) types used throughout this crate
pub mod math {
    pub use glam::Vec2;
}

mod minkowski;
mod simplex;

/// Trait to be implemented by shape types to support collision detection
///
/// The shape implementing this trait **must be convex**.
///
/// # Examples of implementation
///
/// ## Circle
///
/// ```
/// # use gjk2d::{Support, math::Vec2};
/// struct Circle { center: Vec2, radius: f32 };
/// impl Support for Circle {
///     /// Returns the farthest point of the shape in the given direction
///     fn support(&self, direction: Vec2) -> Vec2 {
///         // For a circle, the farthest point, is equal to `direction` with the length clamped at radius length's
///         self.center + direction.clamp_length(self.radius, self.radius)
///     }
/// }
/// ```
///
/// ## Rectangle
///
/// ```
/// # use gjk2d::{Support, math::Vec2};
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
    ///
    /// The given direction length must be finite and greater than zero
    ///
    /// # Panics
    ///
    /// The implementation may panic if the direction is not finite or has a zero length
    fn support(&self, direction: Vec2) -> Vec2;
}

/// Returns true if the two convex shapes geometries are overlapping
pub fn collides(shape1: &impl Support, shape2: &impl Support) -> bool {
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
