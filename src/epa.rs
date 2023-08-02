use core::mem;

use glam::Vec2;
use smallvec::{smallvec, SmallVec};

use crate::{gjk, Contact, Support};

pub(crate) fn generate_contact(
    difference: &impl Support<Vec2, Vec2>,
    simplex: gjk::Simplex,
) -> Contact {
    let mut simplex: Simplex = simplex.into();
    for _ in 0..1000 {
        let edge = simplex.closest_edge();
        let support = difference.support(edge.normal);
        let penetration = support.dot(edge.normal);
        if penetration - edge.distance <= f32::EPSILON {
            return edge.into();
        }
        simplex.insert(edge.index, support);
    }
    panic!("Couldn't generate contact data");
}

struct Edge {
    index: usize,
    normal: Vec2,
    distance: f32,
}

impl From<Edge> for Contact {
    fn from(edge: Edge) -> Self {
        Contact {
            normal: (-edge.normal).into(),
            penetration: edge.distance,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Simplex {
    points: SmallVec<[Vec2; 10]>,
}

impl Simplex {
    fn closest_edge(&self) -> Edge {
        let mut closest_edge = Edge {
            index: 0,
            distance: f32::MAX,
            normal: Vec2::ZERO,
        };

        for index in 0..self.points.len() {
            let p1 = self.points[index];
            let p2 = self
                .points
                .get(index + 1)
                .copied()
                .unwrap_or_else(|| self.points[0]);
            let edge = p2 - p1;
            let outward = edge.perp().normalize_or_zero();
            let distance = p1.dot(outward);
            if distance < closest_edge.distance {
                closest_edge.index = index;
                closest_edge.distance = distance;
                closest_edge.normal = outward;
            }
        }
        closest_edge
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
    use approx::assert_ulps_eq;

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
                points: smallvec![
                    Vec2::Y * 9.0,
                    Vec2::X * 5.0 - Vec2::Y,
                    -Vec2::X * 5.0 - Vec2::Y
                ],
            };
            let Edge {
                index,
                normal,
                distance,
            } = simplex.closest_edge();
            assert_eq!(index, 1);
            assert_ulps_eq!(distance, 1.0);
            assert_eq!(normal, -Vec2::Y);
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
