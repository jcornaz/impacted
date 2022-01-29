use core::cmp::Ordering;
use glam::Vec2;

use crate::{Simplex, Support};

pub(crate) fn find_simplex_enclosing_origin(
    shape: &impl Support,
    mut initial_direction: Vec2,
) -> Option<Simplex> {
    if initial_direction.length_squared() <= f32::EPSILON {
        initial_direction = Vec2::X;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    struct InvalidSupport;
    impl Support for InvalidSupport {
        fn support(&self, _: Vec2) -> Vec2 {
            Vec2::NAN
        }
    }

    #[test]
    fn invalid_support() {
        assert!(find_simplex_enclosing_origin(&InvalidSupport, Vec2::X).is_none());
    }
}
