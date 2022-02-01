#![allow(deprecated)]

#[non_exhaustive]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[deprecated(since = "1.1.0", note = "this error type is not used")]
#[doc(hidden)]
pub enum Error {
    #[cfg_attr(feature = "std", error("non-invertible transform"))]
    NonInvertibleTransform,
}
