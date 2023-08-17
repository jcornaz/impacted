#![cfg(feature = "unstable-v3")]

use approx::assert_abs_diff_eq;
use impacted::v3::{cast_ray, Aabb, Point, Shape, Vec2};
use rstest::rstest;

#[rstest]
#[case(
    Vec2::ZERO,
    Vec2::X,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(1.9, 0.0)),
    Vec2::new(0.9, 0.0)
)]
#[case(
    Vec2::ZERO,
    -Vec2::X,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(-1.9, 0.0)),
    Vec2::new(-0.9, 0.0)
)]
#[case(
    Vec2::X,
    Vec2::X,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(2.9, 0.0)),
    Vec2::new(1.9, 0.0)
)]
#[case(
    Vec2::ZERO,
    Vec2::X * 2.0,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(2.9, 0.0)),
    Vec2::new(1.9, 0.0)
)]
#[case(
    Vec2::ZERO,
    Vec2::Y,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(0.0, 1.9)),
    Vec2::new(0.0, 0.9)
)]
#[case(
    Vec2::Y,
    Vec2::Y,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(0.0, 2.9)),
    Vec2::new(0.0, 1.9)
)]
#[case(
    Vec2::ZERO,
    Vec2::Y * 2.0,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(0.0, 2.9)),
    Vec2::new(0.0, 1.9)
)]
#[case(
    Vec2::ZERO,
    Vec2::new(1.0, 1.0),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(1.9, 1.9)),
    Vec2::new(0.9, 0.9),
)]
#[case(
    Vec2::ZERO,
    Vec2::new(1.0, 1.0),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(0.5, 1.9)),
    Vec2::new(0.9, 0.9),
)]
fn should_find_contact_point(
    #[case] origin: impl Into<Point>,
    #[case] vector: Vec2,
    #[case] target: impl Shape,
    #[case] expected_point: impl Into<Point>,
) {
    let expected_point = expected_point.into();
    let point = cast_ray(origin.into(), vector, &target).unwrap().point;
    assert_abs_diff_eq!(point.x(), expected_point.x());
    assert_abs_diff_eq!(point.y(), expected_point.y());
}

#[rstest]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(2.1, 0.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(-2.1, 0.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::ZERO))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(1.0, 1.0)).with_center_at(Vec2::ZERO))]
#[case(-Vec2::X, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(1.1, 0.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(1.9, 5.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Vec2::new(1.9, -5.0)))]
fn should_return_none_when_there_is_no_hit(
    #[case] origin: impl Into<Point>,
    #[case] vector: Vec2,
    #[case] target: impl Shape,
) {
    let result = cast_ray(origin.into(), vector, &target);
    assert_eq!(result, None);
}
