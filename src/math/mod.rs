mod array;
mod glam_0_24;

pub(crate) trait Dot {
    type Scalar;
    fn dot(self, other: Self) -> Self::Scalar;
}

pub(crate) trait Zero {
    const ZERO: Self;
}

pub(crate) trait IsNegative {
    fn is_negative(&self) -> bool;
}

pub(crate) trait Perp {
    fn perp(self) -> Self;
}
