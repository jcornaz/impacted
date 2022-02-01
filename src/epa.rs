use core::mem;

use glam::Vec2;
use smallvec::{smallvec, SmallVec};

use crate::gjk;

#[derive(Debug, Clone, PartialEq)]
struct Simplex {
    points: SmallVec<[Vec2; 10]>,
}

impl Simplex {
    fn next(&self) -> (usize, Vec2) {
        let mut min_dist = f32::MIN;
        let mut result = (0, Vec2::ZERO);
        for index in 0..self.points.len() {
            let p1 = self.points[index];
            let p2 = self
                .points
                .get(index + 1)
                .copied()
                .unwrap_or_else(|| self.points[0]);
            let edge = p2 - p1;
            let outward = edge.perp().normalize_or_zero();
            let dist = p1.dot(outward);
            if dist > min_dist {
                result = (index, outward);
                min_dist = dist;
            }
        }
        result
    }

    fn insert(&mut self, index: usize, point: Vec2) {
        self.points.insert(index + 1, point);
    }
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
            let expected = [Vec2::ZERO, Vec2::X, Vec2::Y];
            let simplex1: Simplex = gjk::Simplex::Triangle(Vec2::ZERO, Vec2::X, Vec2::Y).into();
            assert_eq!(&simplex1.points[..], &expected);
            let simplex2: Simplex = gjk::Simplex::Triangle(Vec2::ZERO, Vec2::Y, Vec2::X).into();
            assert_eq!(&simplex2.points[..], &expected);
        }

        #[test]
        fn next_returns_feature_index_and_outward_direction() {
            let simplex = Simplex {
                points: smallvec![Vec2::Y * 2.0, Vec2::X - Vec2::Y, -Vec2::X - Vec2::Y],
            };
            let (index, direction) = simplex.next();
            assert_eq!(index, 1);
            assert_eq!(direction.normalize(), -Vec2::Y);
        }

        #[test]
        fn insert_point() {
            let mut simplex = Simplex {
                points: smallvec![Vec2::Y * 2.0, Vec2::X - Vec2::Y, -Vec2::X - Vec2::Y],
            };
            simplex.insert(1, -Vec2::Y);
            assert_eq!(
                &simplex.points[..],
                &[
                    Vec2::Y * 2.0,
                    Vec2::X - Vec2::Y,
                    -Vec2::Y,
                    -Vec2::X - Vec2::Y
                ]
            );
        }
    }
}
