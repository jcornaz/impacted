#![cfg(feature = "std")]

use std::f32::consts;

use glam::{Affine2, Vec2};
use rstest::*;

use gjk2d::{
    prelude::*,
    shapes::{Circle, Rectangle},
};

#[rstest]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Circle::new(1.0),
    Affine2::IDENTITY
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Circle::new(1.0),
    Affine2::from_angle(2.0)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Circle::new(1.0),
    Affine2::from_translation(Vec2::X * 1.0)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Circle::new(1.5),
    Affine2::from_translation(Vec2::Y * 2.1)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Rectangle::from_half_extents(Vec2::new(1.0, 1.0)),
    Affine2::from_translation(Vec2::X * 1.9)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Rectangle::from_half_extents(Vec2::splat(1.0)),
    Affine2::from_angle_translation(consts::FRAC_PI_4, Vec2::X * 2.3)
)]
fn collides(
    #[case] shape1: impl Support,
    #[case] transform1: Affine2,
    #[case] shape2: impl Support,
    #[case] transform2: Affine2,
) {
    assert!(gjk2d::collides(
        &TransformedShape::new(&transform1.try_into().unwrap(), &shape1),
        &TransformedShape::new(&transform2.try_into().unwrap(), &shape2)
    ))
}

#[rstest]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Circle::new(1.0),
    Affine2::from_translation(Vec2::X * 2.1)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Circle::new(1.0),
    Affine2::from_translation(Vec2::Y * 2.1)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Rectangle::from_half_extents(Vec2::new(1.0, 1.0)),
    Affine2::from_translation(Vec2::X * 2.1)
)]
#[case(
    Circle::new(1.0),
    Affine2::IDENTITY,
    Rectangle::from_half_extents(Vec2::splat(1.0)),
    Affine2::from_angle_translation(consts::FRAC_PI_4, Vec2::X * 2.5)
)]
fn does_not_collide(
    #[case] shape1: impl Support,
    #[case] transform1: Affine2,
    #[case] shape2: impl Support,
    #[case] transform2: Affine2,
) {
    assert!(!gjk2d::collides(
        &TransformedShape::new(&transform1.try_into().unwrap(), &shape1),
        &TransformedShape::new(&transform2.try_into().unwrap(), &shape2)
    ))
}
