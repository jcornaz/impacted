#![cfg(feature = "std")]

use std::f32::consts;

use approx::assert_abs_diff_eq;
use glam::Vec2;
use rstest::*;

use impacted::{CollisionShape, Transform};

#[rstest]
#[case(CollisionShape::new_circle(1.0), CollisionShape::new_circle(1.0))]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::ZERO)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_angle_translation(2.0, Vec2::ZERO)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_scale_angle_translation(Vec2::splat(2.0), 2.0, Vec2::ZERO)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::new(2.0, 0.0))),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::X * 1.0)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.5).with_transform(Transform::from_translation(Vec2::Y * 2.1)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_translation(Vec2::X * 1.9)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_angle_translation(consts::FRAC_PI_4, Vec2::X * 2.3)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_segment(Vec2::ZERO, Vec2::X)
)]
fn collides(#[case] shape1: CollisionShape, #[case] shape2: CollisionShape) {
    assert!(shape1.is_collided_with(&shape2));
    let contact = shape1.contact_with(&shape2);
    assert!(contact.is_some(), "{contact:?}");
}

#[rstest]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::X * 2.1)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::Y * 2.1)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_translation(Vec2::X * 2.1)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_angle_translation(consts::FRAC_PI_4, Vec2::X * 2.5)),
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_segment(Vec2::X * 2.0, Vec2::X * 3.0),
)]
fn does_not_collide(#[case] shape1: CollisionShape, #[case] shape2: CollisionShape) {
    assert!(!shape1.is_collided_with(&shape2));
    let contact = shape1.contact_with(&shape2);
    assert!(contact.is_none(), "{contact:?}");
}

#[rstest]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::X * 1.95)),
    Vec2::new(-1.0, 0.0)
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::Y * 1.95)),
    Vec2::new(0.0, -1.0)
)]
#[case(
    CollisionShape::new_rectangle(1.0, 1.0),
    CollisionShape::new_rectangle(1.0, 1.0).with_transform(Transform::from_translation(Vec2::X * -0.95)),
    Vec2::new(1.0, 0.0)
)]
#[case(
    CollisionShape::new_rectangle(2.0, 2.0),
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_angle_translation(consts::FRAC_PI_4 + 0.1, Vec2::X * 2.3)),
    Vec2::new(-1.0, 0.0)
)]
#[case(
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_angle_translation(consts::FRAC_PI_4 + 0.1, Vec2::X * 2.3)),
    CollisionShape::new_rectangle(2.0, 2.0),
    Vec2::new(1.0, 0.0)
)]
fn contact_normal(
    #[case] shape1: CollisionShape,
    #[case] shape2: CollisionShape,
    #[case] expected_normal: Vec2,
) {
    let contact = shape1.contact_with(&shape2).unwrap();
    assert_abs_diff_eq!(Vec2::from(contact.normal), expected_normal, epsilon = 0.001);
}

#[rstest]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::X * 1.95)),
    0.05
)]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_translation(Vec2::Y * 1.0)),
    1.0
)]
#[case(
    CollisionShape::new_rectangle(1.0, 1.0),
    CollisionShape::new_rectangle(1.0, 1.0).with_transform(Transform::from_translation(Vec2::X * -0.95)),
    0.05
)]
#[case(
    CollisionShape::new_rectangle(1.0, 1.0),
    CollisionShape::new_rectangle(1.0, 1.0).with_transform(Transform::from_translation(Vec2::X * -0.5)),
    0.5
)]
#[case(
    CollisionShape::new_rectangle(2.0, 2.0),
    CollisionShape::new_rectangle(2.0, 2.0).with_transform(Transform::from_angle_translation(consts::FRAC_PI_4, Vec2::X * 2.3)),
    0.1142
)]
fn contact_penetration(
    #[case] shape1: CollisionShape,
    #[case] shape2: CollisionShape,
    #[case] expected_penetration: f32,
) {
    let contact = shape1.contact_with(&shape2).unwrap();
    assert_abs_diff_eq!(contact.penetration, expected_penetration, epsilon = 0.0001);
}
