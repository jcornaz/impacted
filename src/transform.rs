use glam::{Affine2, Mat2, Vec2};

use crate::{CollisionShape, Support};

/// Transform that can be used for a [`CollisionShape`]
#[derive(Debug, Clone)]
pub struct Transform {
    local_to_world: Affine2,
    world_to_local: Mat2,
}

impl Transform {
    fn new(local_to_world: Affine2) -> Self {
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

#[cfg(feature = "glam-020")]
impl From<glam::Affine2> for Transform {
    #[inline]
    fn from(affine: Affine2) -> Self {
        Self::new(affine)
    }
}

#[cfg(feature = "bevy-transform-06")]
impl From<bevy_transform_06::components::GlobalTransform> for Transform {
    #[inline]
    fn from(transform: bevy_transform_06::components::GlobalTransform) -> Self {
        Self::from(Affine2::from_scale_angle_translation(
            transform.scale.truncate(),
            angle_2d_from_quat(transform.rotation),
            transform.translation.truncate(),
        ))
    }
}

#[cfg(feature = "bevy-transform-06")]
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

#[cfg(test)]
mod tests {
    use core::f32::consts;

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
        assert_eq!(2.0, support_point.y);
    }

    #[cfg(all(feature = "std", feature = "bevy-transform-06"))]
    mod angle_from_quat {
        use glam::{Quat, Vec3};
        use rstest::rstest;

        use super::*;

        #[rstest]
        #[case(Quat::from_axis_angle(Vec3::Z, 1.0), 1.0)]
        #[case(Quat::from_axis_angle(- Vec3::Z, 1.0), - 1.0)]
        #[case(Quat::from_axis_angle(Vec3::Z, - 1.0), - 1.0)]
        #[case(Quat::from_axis_angle(- Vec3::Z, - 1.0), 1.0)]
        #[cfg(all(feature = "std", feature = "bevy-transform-06"))]
        fn angle_from_quat(#[case] quat: Quat, #[case] expected: f32) {
            assert!((angle_2d_from_quat(quat) - expected).abs() < f32::EPSILON);
        }
    }
}
