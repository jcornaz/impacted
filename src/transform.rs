//! Transform for [`Support`] implementation
//!
//! This module provides a [`Transform`] and [`TransformedShape`]
//! to wrap a shape implementing [`Support`].
//!
//! # Example
//!
//! ```
//! # use core::{f32::consts, convert::TryInto};
//! # use gjk2d::prelude::*;
//! # use gjk2d::glam::{Vec2, Affine2};
//! # fn main() -> Result<(), gjk2d::transform::Error> {
//! // Create a rectangle at the origin, without rotation nor scale
//! let rectangle = shapes::Rectangle::from_half_extents(Vec2::splat(1.0));
//!
//! // A position, rotation and scale for the rectangle
//! let transform: Transform = Affine2::from_scale_angle_translation(
//!     Vec2::splat(2.0), // sized doubled
//!     consts::FRAC_PI_4, // rotated by 45°
//!     Vec2::new(1., 2.), // translated
//! ).try_into()?; // <-- Only reversible matrices can successfully be converted into a `Transform`
//!
//! // Get a transformed implementation of the `Support` trait
//! let transformed = TransformedShape::new(&transform, &rectangle);
//!
//! let support_point = transformed.support(Vec2::X);
//! assert!((3.5 .. 4.0).contains(&support_point.x));
//! assert_eq!(2.0, support_point.y);
//! # Ok(()) }
//! ```

use crate::{
    glam::{Affine2, Mat2, Vec2},
    Support,
};

/// Error returned if trying to create a [`Transform`] from a non-reversible matrix
#[non_exhaustive]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[cfg_attr(feature = "std", error("non-reversible transform"))]
pub struct Error;

/// Transform that can be used to get a [`TransformedShape`]
///
/// See [`transform`](self) module documentation for more details and examples
#[derive(Debug, Clone)]
pub struct Transform {
    local_to_world: Affine2,
    world_to_local: Mat2,
}

impl Default for Transform {
    /// The default transform is the identity transform
    fn default() -> Self {
        Self {
            local_to_world: Affine2::IDENTITY,
            world_to_local: Mat2::IDENTITY,
        }
    }
}

impl TryFrom<Affine2> for Transform {
    type Error = Error;

    /// Try to create a transform from the glam `Affine2`
    ///
    /// It returns an error if the affine is not reversible
    ///
    /// # Example
    ///
    /// ```
    /// # use core::convert::TryFrom;
    /// # use gjk2d::prelude::*;
    /// # use gjk2d::glam::{Vec2, Affine2};
    /// assert!(Transform::try_from(Affine2::IDENTITY).is_ok());
    /// assert!(Transform::try_from(Affine2::from_scale(Vec2::ZERO)).is_err());
    /// ```
    fn try_from(local_to_world: Affine2) -> Result<Self, Self::Error> {
        let world_to_local = local_to_world.matrix2.inverse();
        if world_to_local.is_nan() {
            Err(Error)
        } else {
            Ok(Self {
                local_to_world,
                world_to_local,
            })
        }
    }
}

#[cfg(feature = "bevy_transform")]
impl TryFrom<bevy_transform::components::GlobalTransform> for Transform {
    type Error = Error;

    /// Try to create a transform from the bevy `GlobalTransform`
    ///
    /// It returns an error if the transform is not reversible
    ///
    /// # Example
    ///
    /// ```
    /// # use core::convert::TryFrom;
    /// # use glam::Vec3;
    /// use bevy_transform::prelude::{GlobalTransform, Transform as BevyTransform};
    /// use gjk2d::transform::Transform;
    /// assert!(Transform::try_from(GlobalTransform::default()).is_ok());
    /// assert!(Transform::try_from(GlobalTransform::from(BevyTransform::from_scale(Vec3::ZERO))).is_err());
    /// ```
    fn try_from(
        transform: bevy_transform::components::GlobalTransform,
    ) -> Result<Self, Self::Error> {
        Self::try_from(Affine2::from_scale_angle_translation(
            transform.scale.truncate(),
            angle_2d_from_quat(transform.rotation),
            transform.translation.truncate(),
        ))
    }
}

/// Association of a [`Transform`] and a [`Support`] references
///
/// See [`transform`](self) module documentation for more details and examples
pub struct TransformedShape<'a, S> {
    transform: &'a Transform,
    shape: &'a S,
}

impl<'a, S: Support> TransformedShape<'a, S> {
    /// Create a transformed shape
    #[inline]
    #[must_use]
    pub fn new(transform: &'a Transform, shape: &'a S) -> Self {
        Self { transform, shape }
    }
}

impl<'a, S: Support> Support for TransformedShape<'a, S> {
    fn support(&self, direction: Vec2) -> Vec2 {
        let local_direction = self.transform.world_to_local * direction;
        let local_support = self.shape.support(local_direction);
        self.transform
            .local_to_world
            .transform_point2(local_support)
    }
}

#[cfg(feature = "bevy_transform")]
fn angle_2d_from_quat(quat: glam::Quat) -> f32 {
    if quat.is_near_identity() {
        return 0.0;
    }
    let projected = quat.to_scaled_axis().project_onto(glam::Vec3::Z);
    let angle = projected.length();
    if projected.z < 0.0 {
        -angle
    } else {
        angle
    }
}

#[cfg(all(test, feature = "bevy_transform", feature = "std"))]
mod tests {
    use glam::{Quat, Vec3};
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(Quat::from_axis_angle(Vec3::Z, 1.0), 1.0)]
    #[case(Quat::from_axis_angle(-Vec3::Z, 1.0), -1.0)]
    #[case(Quat::from_axis_angle(Vec3::Z, -1.0), -1.0)]
    #[case(Quat::from_axis_angle(-Vec3::Z, -1.0), 1.0)]
    fn angle_from_quat(#[case] quat: Quat, #[case] expected: f32) {
        assert_eq!(angle_2d_from_quat(quat), expected);
    }
}
