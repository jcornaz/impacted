#![cfg(feature = "std")]

use std::f32::consts;

use glam::Vec2;
use rstest::*;

use impacted::{CollisionShape, Transform};

#[rstest]
#[case(CollisionShape::new_circle(1.0), CollisionShape::new_circle(1.0))]
#[case(
    CollisionShape::new_circle(1.0),
    CollisionShape::new_circle(1.0).with_transform(Transform::from_angle_translation(2.0, Vec2::ZERO)),
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
    assert!(shape1.is_collided_with(&shape2))
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
    assert!(!shape1.is_collided_with(&shape2))
}
