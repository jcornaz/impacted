mod array;

pub(crate) trait Dot {
    type Scalar;
    fn dot(self, other: Self) -> Self::Scalar;
}
