//! Bounds representing an axis aligned bounding box.

use glam::Vec2;

use crate::Transform;

/// Represents an axis aligned bounding box.
pub struct Bounds {
    half_extents: Vec2,
    transform: Transform,
}

impl Bounds {
    /// Creates a new bounding box from a translation and half extents
    pub fn new(translation: impl Into<[f32; 2]>, half_extents: Vec2) -> Self {
        Bounds {
            half_extents,
            transform: Transform::from_translation(translation),
        }
    }

    /// The center of the bounding box
    #[must_use]
    pub fn center(&self) -> Vec2 {
        self.transform.position()
    }

    /// The extents of the bounding box. This is always half of the size of the Bounds.
    #[must_use]
    pub fn extends(&self) -> Vec2 {
        self.half_extents
    }

    /// The minimal point of the box. This is always equal to center - extents.
    #[must_use]
    pub fn min(&self) -> Vec2 {
        self.transform.position() - self.half_extents
    }

    /// The maximal point of the box. This is always equal to center + extents.
    #[must_use]
    pub fn max(&self) -> Vec2 {
        self.transform.position() + self.half_extents
    }

    /// The total size of the box. This is always twice as large as the extents.
    #[must_use]
    pub fn size(&self) -> Vec2 {
        self.transform.position() + self.half_extents
    }
}
