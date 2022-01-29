use glam::Affine2;

use crate::Transform;

impl From<Affine2> for Transform {
    #[inline]
    fn from(affine: Affine2) -> Self {
        Self::new(affine)
    }
}
