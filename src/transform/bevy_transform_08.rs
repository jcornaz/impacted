use bevy_math_08::{Quat, Vec3};
use bevy_transform_08::prelude::GlobalTransform;

#[cfg(feature = "bevy-transform-08")] // Repeated to appear in the docs
impl From<GlobalTransform> for crate::Transform {
    #[inline]
    fn from(transform: GlobalTransform) -> Self {
        let (scale, rotation, translation) = transform.to_scale_rotation_translation();
        Self::from_scale_angle_translation(
            scale.truncate(),
            angle_2d_from_quat(rotation),
            translation.truncate(),
        )
    }
}

fn angle_2d_from_quat(quat: Quat) -> f32 {
    if quat.is_near_identity() {
        return 0.0;
    }
    let projected = quat.to_scaled_axis().project_onto(Vec3::Z);
    let angle = projected.length();
    if projected.z < 0.0 {
        -angle
    } else {
        angle
    }
}

#[cfg(all(test, feature = "std"))]
mod angle_from_quat {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Quat::from_axis_angle(Vec3::Z, 1.0), 1.0)]
    #[case(Quat::from_axis_angle(- Vec3::Z, 1.0), - 1.0)]
    #[case(Quat::from_axis_angle(Vec3::Z, - 1.0), - 1.0)]
    #[case(Quat::from_axis_angle(- Vec3::Z, - 1.0), 1.0)]
    fn angle_from_quat(#[case] quat: Quat, #[case] expected: f32) {
        assert!((angle_2d_from_quat(quat) - expected).abs() < f32::EPSILON);
    }
}
