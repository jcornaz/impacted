#![cfg(feature = "unstable-v3-aabb")]

use approx::assert_abs_diff_eq;
use impacted::v3::{ray_cast, Aabb, Point, Shape, Vec2};
use rstest::rstest;

#[rstest]
#[case(
    Vec2::ZERO,
    Vec2::X,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 0.0)),
    Vec2::new(0.9, 0.0),
)]
#[case(
    Vec2::ZERO,
    -Vec2::X,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(-1.9, 0.0)),
    Vec2::new(-0.9, 0.0)
)]
#[case(
    Vec2::X,
    Vec2::X,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(2.9, 0.0)),
    Vec2::new(1.9, 0.0)
)]
#[case(
    Vec2::ZERO,
    Vec2::X * 2.0,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(2.9, 0.0)),
    Vec2::new(1.9, 0.0)
)]
#[case(
    Vec2::ZERO,
    Vec2::Y,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, 1.9)),
    Vec2::new(0.0, 0.9)
)]
#[case(
    Vec2::Y,
    Vec2::Y,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, 2.9)),
    Vec2::new(0.0, 1.9)
)]
#[case(
    Vec2::ZERO,
    Vec2::Y * 2.0,
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, 2.9)),
    Vec2::new(0.0, 1.9)
)]
#[case(
    Vec2::ZERO,
    Vec2::new(1.0, 1.0),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 1.9)),
    Vec2::new(0.9, 0.9),
)]
#[case(
    Vec2::ZERO,
    Vec2::new(1.0, 1.0),
    Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.5, 1.9)),
    Vec2::new(0.9, 0.9),
)]
fn should_find_contact_time(
    #[case] origin: impl Into<Point>,
    #[case] vector: Vec2,
    #[case] target: impl Shape,
    #[case] expected_point: impl Into<Point>,
) {
    let origin = origin.into();
    let expected_point = expected_point.into();
    let time = ray_cast(origin, vector, &target).unwrap().time;
    let point = origin + (vector * time);
    assert_abs_diff_eq!(point.x(), expected_point.x());
    assert_abs_diff_eq!(point.y(), expected_point.y());
}

#[rstest]
#[case(Point::ORIGIN, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 0.0)), -Vec2::X)]
#[case(Point::ORIGIN, -Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(-1.9, 0.0)), Vec2::X)]
#[case(Point::ORIGIN, Vec2::Y, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, 1.9)), -Vec2::Y)]
#[case(Point::ORIGIN, -Vec2::Y, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(0.0, -1.9)), Vec2::Y)]
#[case(Point::ORIGIN, Vec2::new(1.0, 1.0), Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 0.0)), -Vec2::X)]
fn should_find_contact_normal(
    #[case] origin: impl Into<Point>,
    #[case] vector: Vec2,
    #[case] target: impl Shape,
    #[case] expected_normal: Vec2,
) {
    let origin = origin.into();
    let normal = ray_cast(origin, vector, &target).unwrap().normal;
    assert_abs_diff_eq!(normal.x(), expected_normal.x());
    assert_abs_diff_eq!(normal.y(), expected_normal.y());
}

#[rstest]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(2.1, 0.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(-2.1, 0.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::ORIGIN))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(1.0, 1.0)).with_center_at(Point::ORIGIN))]
#[case(-Vec2::X, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.1, 0.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, 5.0)))]
#[case(Vec2::ZERO, Vec2::X, Aabb::from_size(Vec2::new(2.0, 2.0)).with_center_at(Point::new(1.9, -5.0)))]
fn should_return_none_when_there_is_no_hit(
    #[case] origin: impl Into<Point>,
    #[case] vector: Vec2,
    #[case] target: impl Shape,
) {
    let result = ray_cast(origin.into(), vector, &target);
    assert_eq!(result, None);
}
