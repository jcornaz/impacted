use glam::Vec2;

use crate::{Simplex, Support};

pub(crate) fn find_simplex_enclosing_origin(
    shape: &impl Support,
    initial_direction: Vec2,
) -> Option<Simplex> {
    let mut simplex = {
        let first_point = shape.support(initial_direction);
        if first_point.dot(initial_direction) <= 0.0 {
            return None;
        }
        Simplex::new(first_point)
    };

    while let Some(direction) = simplex.next() {
        let point = shape.support(direction);
        if point.dot(direction) <= 0.0 {
            return None;
        }
        simplex.insert(point);
    }
    Some(simplex)
}
