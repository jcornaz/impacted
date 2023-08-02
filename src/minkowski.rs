use core::ops::{Neg, Sub};

use crate::Support;

pub(crate) struct Difference<'a, S1, S2> {
    pub(crate) shape1: &'a S1,
    pub(crate) shape2: &'a S2,
}

impl<V, S1, S2> Support<V> for Difference<'_, S1, S2>
where
    V: Copy + Sub<Output = V> + Neg<Output = V> + Into<V>,
    S1: Support<V>,
    S2: Support<V>,
{
    fn support(&self, direction: V) -> V {
        self.shape1.support(direction) - self.shape2.support(-direction)
    }
}
