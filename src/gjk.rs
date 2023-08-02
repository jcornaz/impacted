use core::cmp::Ordering;

use glam::Vec2;

use crate::Support;

pub(crate) fn find_simplex_enclosing_origin(
    shape: &impl Support<Vec2, Vec2>,
    initial_direction: Vec2,
) -> Option<Simplex> {
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
pub(crate) enum Simplex {
    Point(Vec2),
    Line(Vec2, Vec2),
    Triangle(Vec2, Vec2, Vec2),
}

impl Simplex {
    pub(crate) fn new(point: Vec2) -> Self {
        Self::Point(point)
    }

    pub(crate) fn insert(&mut self, new_point: Vec2) {
        match *self {
            Self::Point(p) => *self = Self::Line(p, new_point),
            Self::Line(p1, p2) => *self = Self::Triangle(p1, p2, new_point),
            Self::Triangle(_, _, _) => {
                panic!("Cannot expand 2d simplex further than triangle")
            }
        }
    }

    /// Set to the simpler simplex that is closest to the origin.
    ///
    /// If the origin is inside the simplex returns None. Otherwise returns the next direction to test.
    pub(crate) fn next(&mut self) -> Option<Vec2> {
        match *self {
            Self::Point(point) => {
                if point.length_squared() > f32::EPSILON {
                    Some(-point)
                } else {
                    None
                }
            }
            Self::Line(p1, p2) => {
                let mut dir = (p2 - p1).perp();
                if dir.dot(p1) > 0.0 {
                    dir = -dir;
                }
                if dir.dot(p1) < -f32::EPSILON {
                    Some(dir)
                } else {
                    None
                }
            }
            Self::Triangle(p1, p2, p3) => {
                let mut dir = perp(p3 - p1, p3 - p2);
                if dir.dot(-p3) > 0.0 {
                    *self = Self::Line(p1, p3);
                    return Some(dir);
                }
                dir = perp(p3 - p2, p3 - p1);
                if dir.dot(-p3) > 0.0 {
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
fn perp(axis: Vec2, direction: Vec2) -> Vec2 {
    let perp = axis.perp();
    if perp.dot(direction) >= 0.0 {
        perp
    } else {
        -perp
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use rstest::rstest;

    use super::*;

    struct InvalidSupport;
    impl Support<Vec2, Vec2> for InvalidSupport {
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
    fn contains_origin(#[case] simplex: Simplex) {
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
