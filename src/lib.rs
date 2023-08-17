#![deny(future_incompatible)]
#![warn(nonstandard_style, rust_2018_idioms, missing_docs, clippy::pedantic)]
#![deny(unsafe_code)]
#![allow(clippy::wildcard_imports)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! 2d collision test for game-development in rust
//!
//! This provides a low-level "narrow-phase" collision-detection logic.
//!
//! If you want to pair it with a broad-phase, you may look at [bvh-arena] or [broccoli].
//!
//! [bvh-arena]: https://github.com/jcornaz/bvh-arena
//! [broccoli]: https://github.com/tiby312/broccoli
//!
//! # Usage
//!
//! The central type is [`CollisionShape`]. Once a collision shape is created and positioned (with a [`Transform`])
//! is is possible to call [`CollisionShape::is_collided_with`] to test for collision with another shape.
//!
//! ```
//! # use approx::assert_ulps_eq;
//! use impacted::{CollisionShape, Transform, Contact};
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
//!
//! // And generate contact data
//! // (It returns `None` if there is no contact)
//! let contact = circle.contact_with(&rect1).unwrap();
//! let normal: Vec2 = contact.normal.into();
//! assert_ulps_eq!(normal, -Vec2::X);
//! assert_ulps_eq!(contact.penetration, 1.0);
//! ```
//!
//! ## Feature flags
//!
//! * `std` (enabled by default) Allow to use rust the standard library (need to be disabled for `no_std` apps)
//! * `bvh-arena` Integration with [bvh-arena](https://crates.io/crates/bvh-arena) bounding volumes
//!
//!
//! ## Unstable feature flags
//!
//! The following features may receive breaking changes or be removed in a patch release.
//!
//! * `unstable-v3` `v3` module, an exploration of what could be the next major version of the API
//!

#[cfg(all(test, feature = "unstable-v3"))]
extern crate alloc;

mod v2;
#[cfg(feature = "unstable-v3")]
pub mod v3;

pub use v2::*;
