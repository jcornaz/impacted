mod math;

use math::Vec2;
use sealed::sealed;
use shapes::Aabb;

trait ConvexPolygon {
    fn axes(&self) -> &[Vec2];
}

#[sealed]
trait Collides<Rhs> {
    fn collides(&self, other: &Rhs) -> bool;
}

trait AxisProjection {
    fn project(&self, axis: Vec2) -> (f32, f32);
}

mod shapes {
    use super::{math::Vec2, AxisProjection};

    pub(super) struct Aabb {
        half_size: Vec2,
    }

    impl Aabb {
        fn from_size(size: Vec2) -> Self {
            Self {
                half_size: size / 2.0,
            }
        }
    }

    impl AxisProjection for Aabb {
        fn project(&self, axis: Vec2) -> (f32, f32) {
            let p = self.half_size.dot(axis);
            (-p, p)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use approx::assert_abs_diff_eq;
        use rstest::rstest;

        #[rstest]
        #[case(Aabb::from_size(Vec2::new(0.0, 0.0)), Vec2::new(1.0, 0.0), (0.0, 0.0))]
        #[case(Aabb::from_size(Vec2::new(2.0, 0.0)), Vec2::new(1.0, 0.0), (-1.0, 1.0))]
        #[case(Aabb::from_size(Vec2::new(0.0, 2.0)), Vec2::new(0.0, 1.0), (-1.0, 1.0))]
        #[case(Aabb::from_size(Vec2::new(2.0, 2.0)), Vec2::new(0.0, -1.0), (1.0, -1.0))]
        fn test_axis_project(
            #[case] shape: Aabb,
            #[case] axis: Vec2,
            #[case] expected: (f32, f32),
        ) {
            let (min, max) = shape.project(axis);
            assert_abs_diff_eq!(min, expected.0);
            assert_abs_diff_eq!(max, expected.1);
        }
    }
}
