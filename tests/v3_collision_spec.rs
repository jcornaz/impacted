#![cfg(feature = "unstable-v3-aabb")]

use rstest::rstest;

use impacted::v3::{check_collision, Aabb, Point, Shape, Vec2};

#[rstest]
#[case(
    Aabb::from_size(Vec2::new(2.0, 2.0)),
    Aabb::from_size(Vec2::new(2.0, 2.0))
)]
#[case(
    Aabb::from_size(Vec2::new(2.0, 2.0)),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 0.0))
)]
#[case(
    Aabb::from_size(Vec2::new(2.0, 2.0)),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, 1.9))
)]
#[case(Point::ORIGIN, Aabb::from_size(Vec2::new(2.0, 2.0)))]
#[case(
    Point::new(10.9, 9.1),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(10.0, 10.0))
)]
fn test_collides(#[case] shape1: impl Shape, #[case] shape2: impl Shape) {
    assert!(check_collision(&shape1, &shape2));
}

#[rstest]
#[case(
    Aabb::from_size(Vec2::new(2.0, 2.0)),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(2.1, 0.0))
)]
#[case(
    Aabb::from_size(Vec2::new(2.0, 2.0)),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, 2.1))
)]
#[case(
    Point::new(12.1, 9.1),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(10.0, 10.0))
)]
fn test_not_collides(#[case] shape1: impl Shape, #[case] shape2: impl Shape) {
    assert!(!check_collision(&shape1, &shape2));
}
