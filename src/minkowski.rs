use glam::Vec2;

use crate::Support;

pub(crate) struct Difference<'a, S1: Support, S2: Support> {
    pub(crate) shape1: &'a S1,
    pub(crate) shape2: &'a S2,
}

impl<S1: Support, S2: Support> Support for Difference<'_, S1, S2> {
    fn support(&self, direction: Vec2) -> Vec2 {
        self.shape1.support(direction) - self.shape2.support(-direction)
    }
}
