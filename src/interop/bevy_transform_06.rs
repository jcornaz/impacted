use bevy_transform_06::prelude::GlobalTransform;

#[cfg(feature = "bevy-transform-06")] // Repeated to appear in the docs
impl From<GlobalTransform> for crate::Transform {
    #[inline]
    fn from(transform: bevy_transform_06::components::GlobalTransform) -> Self {
        Self::from_scale_angle_translation(
            transform.scale.truncate(),
            angle_2d_from_quat(transform.rotation),
            transform.translation.truncate(),
        )
    }
}

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

#[cfg(all(test, feature = "std"))]
mod angle_from_quat {
    use glam::{Quat, Vec3};
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
