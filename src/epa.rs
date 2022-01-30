use core::mem;

use glam::Vec2;
use smallvec::{smallvec, SmallVec};

use crate::gjk;

#[derive(Debug, Clone, PartialEq)]
struct Simplex {
    points: SmallVec<[Vec2; 7]>,
}

impl From<gjk::Simplex> for Simplex {
    fn from(simplex: gjk::Simplex) -> Self {
        Self {
            points: match simplex {
                gjk::Simplex::Point(p) => smallvec![p],
                gjk::Simplex::Line(p1, p2) => smallvec![p1, p2],
                gjk::Simplex::Triangle(p1, mut p2, mut p3) => {
                    if (p2 - p1).perp_dot(p3 - p2) < 0.0 {
                        mem::swap(&mut p2, &mut p3);
                    }
                    smallvec![p1, p2, p3]
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod simplex {
        use super::*;

        #[test]
        fn starts_with_left_winding() {
            let expected: SmallVec<[Vec2; 7]> = smallvec![Vec2::ZERO, Vec2::X, Vec2::Y];
            let simplex1: Simplex = gjk::Simplex::Triangle(Vec2::ZERO, Vec2::X, Vec2::Y).into();
            assert_eq!(simplex1.points, expected);
            let simplex2: Simplex = gjk::Simplex::Triangle(Vec2::ZERO, Vec2::Y, Vec2::X).into();
            assert_eq!(simplex2.points, expected);
        }
    }
}
