use core::ops::{Neg, Sub};

use crate::Support;

pub(crate) struct Difference<'a, S1, S2> {
    pub(crate) shape1: &'a S1,
    pub(crate) shape2: &'a S2,
}

impl<P, V, S1, S2> Support<P, V> for Difference<'_, S1, S2>
where
    P: Sub<P, Output = V> + From<V>,
    V: Copy + Neg<Output = V>,
    S1: Support<P, V>,
    S2: Support<P, V>,
{
    fn support(&self, direction: V) -> P {
        (self.shape1.support(direction) - self.shape2.support(-direction)).into()
    }
}
