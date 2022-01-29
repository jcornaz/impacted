use glam::Affine2;

use crate::Transform;

#[cfg(feature = "glam-020")] // Repeated to appear in the docs
impl From<Affine2> for Transform {
    #[inline]
    fn from(affine: Affine2) -> Self {
        Self::new(affine)
    }
}
