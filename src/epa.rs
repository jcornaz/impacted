use core::{
    mem,
    ops::{Neg, Sub},
};

use glam::Vec2;
use smallvec::{smallvec, SmallVec};

use crate::{gjk, math::*, Contact, Support};

pub(crate) fn generate_contact(
    difference: &impl Support<Vec2>,
    simplex: gjk::Simplex<Vec2>,
) -> Contact {
    let mut simplex: Simplex<Vec2> = simplex.into();
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

struct Edge<V: Dot> {
    index: usize,
    normal: V,
    distance: <V as Dot>::Output,
}

impl<V> From<Edge<V>> for Contact
where
    V: Neg<Output = V> + Into<[f32; 2]> + Dot<Output = f32>,
{
    fn from(edge: Edge<V>) -> Self {
        Contact {
            normal: (-edge.normal).into(),
            penetration: edge.distance,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Simplex<V> {
    points: SmallVec<[V; 10]>,
}

impl<V> Simplex<V>
where
    V: Dot + Copy + Sub<V, Output = V> + Perp + Normalize + Default,
    <V as Dot>::Output: PartialOrd,
{
    fn closest_edge(&self) -> Edge<V> {
        (0..self.points.len())
            .map(|index| self.edge(index))
            .min_by(|e1, e2| {
                e1.distance
                    .partial_cmp(&e2.distance)
                    .unwrap_or(core::cmp::Ordering::Equal)
            })
            .expect("no edge in epa simplex")
    }

    fn edge(&self, index: usize) -> Edge<V> {
        let p1 = self.points[index];
        let p2 = self
            .points
            .get(index + 1)
            .copied()
            .unwrap_or_else(|| self.points[0]);
        let edge = p2 - p1;
        let normal = edge
            .perp()
            .normalize()
            .or_else(|| p1.normalize())
            .unwrap_or_default();
        let distance = p1.dot(normal);
        Edge {
            index,
            normal,
            distance,
        }
    }
}

impl<V> Simplex<V> {
    fn insert(&mut self, index: usize, point: V) {
        self.points.insert(index + 1, point);
    }
}

impl<V> From<gjk::Simplex<V>> for Simplex<V>
where
    V: Copy + Sub<V, Output = V> + Cross,
    <V as Cross>::Output: CmpToZero,
{
    fn from(simplex: gjk::Simplex<V>) -> Self {
        Self {
            points: match simplex {
                gjk::Simplex::Point(p) => smallvec![p],
                gjk::Simplex::Line(p1, p2) => smallvec![p1, p2],
                gjk::Simplex::Triangle(p1, mut p2, mut p3) => {
                    if (p2 - p1).cross(p3 - p2).is_negative() {
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
            let simplex1: Simplex<Vec2> =
                gjk::Simplex::Triangle(Vec2::ZERO, Vec2::X, Vec2::Y).into();
            assert_eq!(&simplex1.points[..], &expected);
            let simplex2: Simplex<Vec2> =
                gjk::Simplex::Triangle(Vec2::ZERO, Vec2::Y, Vec2::X).into();
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
