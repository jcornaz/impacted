use core::{
    cmp::Ordering,
    ops::{Neg, Sub},
};

use glam::Vec2;

use crate::{math::*, Support};

pub(crate) fn find_simplex_enclosing_origin(
    shape: &impl Support<Vec2>,
    initial_direction: Vec2,
) -> Option<Simplex<Vec2>> {
    let mut simplex = {
        let first_point = shape.support(initial_direction);
        if first_point
            .dot(initial_direction)
            .partial_cmp(&0.0)
            .unwrap_or(Ordering::Less)
            .is_lt()
        {
            return None;
        }
        Simplex::new(first_point)
    };

    while let Some(direction) = simplex.next() {
        let point = shape.support(direction);
        if point
            .dot(direction)
            .partial_cmp(&0.0)
            .unwrap_or(Ordering::Less)
            .is_lt()
        {
            return None;
        }
        simplex.insert(point);
    }
    Some(simplex)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Simplex<P> {
    Point(P),
    Line(P, P),
    Triangle(P, P, P),
}

impl<P> Simplex<P> {
    pub(crate) fn new(point: P) -> Self {
        Self::Point(point)
    }
}

impl<P: Copy> Simplex<P> {
    pub(crate) fn insert(&mut self, new_point: P) {
        match *self {
            Self::Point(p) => *self = Self::Line(p, new_point),
            Self::Line(p1, p2) => *self = Self::Triangle(p1, p2, new_point),
            Self::Triangle(_, _, _) => {
                panic!("Cannot expand 2d simplex further than triangle")
            }
        }
    }
}

impl<V> Simplex<V>
where
    V: Copy + Dot + Perp + Neg<Output = V> + Sub<V, Output = V>,
    <V as Dot>::Scalar: CmpToZero,
{
    /// Set to the simpler simplex that is closest to the origin.
    ///
    /// If the origin is inside the simplex returns None. Otherwise returns the next direction to test.
    pub(crate) fn next(&mut self) -> Option<V> {
        match *self {
            Self::Point(point) => {
                if point.magnitude_squared().is_positive() {
                    Some(-point)
                } else {
                    None
                }
            }
            Self::Line(p1, p2) => {
                let mut dir = (p2 - p1).perp();
                if dir.dot(p1).is_positive() {
                    dir = -dir;
                }
                if dir.dot(p1).is_negative() {
                    Some(dir)
                } else {
                    None
                }
            }
            Self::Triangle(p1, p2, p3) => {
                let mut dir = perp(p3 - p1, p3 - p2);
                if dir.dot(-p3).is_positive() {
                    *self = Self::Line(p1, p3);
                    return Some(dir);
                }
                dir = perp(p3 - p2, p3 - p1);
                if dir.dot(-p3).is_positive() {
                    *self = Self::Line(p2, p3);
                    Some(dir)
                } else {
                    None
                }
            }
        }
    }
}

/// Returns a perpendicular to `axis` that has a positive dot product with `direction`
fn perp<V>(axis: V, direction: V) -> V
where
    V: Copy + Perp + Neg<Output = V> + Dot,
    <V as Dot>::Scalar: CmpToZero,
{
    let perp = axis.perp();
    if perp.dot(direction).is_negative() {
        -perp
    } else {
        perp
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use rstest::rstest;

    use super::*;

    struct InvalidSupport;
    impl Support<Vec2> for InvalidSupport {
        fn support(&self, _: Vec2) -> Vec2 {
            Vec2::NAN
        }
    }

    #[test]
    fn invalid_support() {
        assert!(find_simplex_enclosing_origin(&InvalidSupport, Vec2::X).is_none());
    }

    #[rstest]
    #[case(Simplex::Point(Vec2::default()))]
    #[case(Simplex::Line(-Vec2::X, Vec2::X))]
    #[case(Simplex::Line(Vec2::X, Vec2::default()))]
    #[case(Simplex::Line(Vec2::default(), Vec2::Y))]
    #[case(Simplex::Triangle(Vec2::X, Vec2::Y, Vec2::default()))]
    #[case(Simplex::Triangle(Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0), Vec2::Y))]
    #[cfg(feature = "std")]
    fn contains_origin(#[case] simplex: Simplex<Vec2>) {
        let mut modified = simplex;
        assert!(modified.next().is_none());
        assert_eq!(modified, simplex);
    }

    #[test]
    fn point() {
        let mut simplex = Simplex::new(Vec2::X);
        assert_eq!(simplex.next(), Some(-Vec2::X));
        assert_eq!(simplex, Simplex::new(Vec2::X));
    }

    #[test]
    fn line() {
        let mut simplex = Simplex::Line(Vec2::Y, Vec2::new(1.0, 1.0));
        assert_eq!(simplex.next(), Some(-Vec2::Y));
        assert_eq!(simplex, Simplex::Line(Vec2::Y, Vec2::new(1.0, 1.0)));
    }

    #[test]
    fn triangle() {
        let mut simplex = Simplex::Triangle(Vec2::Y, Vec2::new(1.0, 1.0), Vec2::X);
        assert_eq!(
            simplex.next().map(glam::Vec2::normalize),
            Some(Vec2::new(-1.0, -1.0).normalize())
        );
    }
}
