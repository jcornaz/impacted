use glam::{Affine2, Mat2, Vec2};

use crate::{CollisionShape, Support};

/// Transform that can be used for a [`CollisionShape`]
#[derive(Debug, Clone)]
pub struct Transform {
    local_to_world: Affine2,
    world_to_local: Mat2,
}

impl Transform {
    pub(crate) fn new(local_to_world: Affine2) -> Self {
        let world_to_local = local_to_world.matrix2.inverse();
        Self {
            local_to_world,
            world_to_local,
        }
    }

    /// Create a translation transform
    ///
    /// # Panics
    ///
    /// Panic if the translation is not finite
    ///
    /// # Example with glam
    ///
    /// ```rust
    /// use impacted::Transform;
    /// use glam::Vec2;
    /// let translation = Transform::from_translation(Vec2::new(1.0, 2.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_translation(translation: impl Into<[f32; 2]>) -> Self {
        Self::new(Affine2::from_translation(translation.into().into()))
    }

    /// Create a translation and rotation transform
    ///
    /// # Panics
    ///
    /// Panic if the translation or angle is not finite
    ///
    /// # Example with glam
    ///
    /// ```rust
    /// use impacted::Transform;
    /// use core::f32::consts;
    /// use glam::Vec2;
    /// let translation = Transform::from_angle_translation(consts::FRAC_PI_4, Vec2::new(1.0, 2.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: f32, translation: impl Into<[f32; 2]>) -> Self {
        Self::new(Affine2::from_angle_translation(
            angle,
            translation.into().into(),
        ))
    }

    /// Create a translation, rotation and scale transform
    ///
    /// The scale must not be zero
    ///
    /// # Panics
    ///
    /// Panic if a component of the scale is zero, or if the translation or angle is not finite
    ///
    /// # Example with glam
    ///
    /// ```rust
    /// use impacted::Transform;
    /// use core::f32::consts;
    /// use glam::Vec2;
    /// let translation = Transform::from_scale_angle_translation(
    ///     Vec2::splat(2.0),
    ///     consts::FRAC_PI_4,
    ///     Vec2::new(1.0, 2.0)
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_scale_angle_translation(
        scale: impl Into<[f32; 2]>,
        angle: f32,
        translation: impl Into<[f32; 2]>,
    ) -> Self {
        Self::new(Affine2::from_scale_angle_translation(
            scale.into().into(),
            angle,
            translation.into().into(),
        ))
    }

    pub(crate) fn position(&self) -> Vec2 {
        self.local_to_world.translation
    }
}

impl Default for Transform {
    /// The default transform is the identity transform
    #[inline]
    fn default() -> Self {
        Self {
            local_to_world: Affine2::IDENTITY,
            world_to_local: Mat2::IDENTITY,
        }
    }
}

impl Support for CollisionShape {
    fn support(&self, direction: Vec2) -> Vec2 {
        let local_direction = self.transform.world_to_local * direction;
        let local_support = self.data.support(local_direction);
        self.transform
            .local_to_world
            .transform_point2(local_support)
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts;

    use approx::assert_ulps_eq;
    use glam::Vec2;

    use super::*;

    #[test]
    fn transformed_shape_support() {
        let transform: Transform = Transform::from_scale_angle_translation(
            Vec2::splat(2.0),  // sized doubled
            consts::FRAC_PI_4, // rotated by 45°
            Vec2::new(1., 2.), // translated
        );

        let support_point = CollisionShape::new_rectangle(2.0, 2.0)
            .with_transform(transform)
            .support(Vec2::X);
        assert!((3.5..4.0).contains(&support_point.x));
        assert_ulps_eq!(2.0, support_point.y);
    }
}
