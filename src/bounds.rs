//! Bounds representing an axis aligned bounding box.

use glam::Vec2;

/// Represents an axis aligned bounding box.
#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    half_extents: Vec2,
    position: Vec2,
}

impl Bounds {
    /// Creates a new bounding box from a translation and half extents
    #[must_use]
    pub fn new(position: Vec2, half_extents: Vec2) -> Self {
        Bounds {
            half_extents,
            position,
        }
    }

    /// The center of the bounding box
    #[must_use]
    pub fn center(&self) -> Vec2 {
        self.position
    }

    /// The extents of the bounding box. This is always half of the size of the Bounds.
    #[must_use]
    pub fn extends(&self) -> Vec2 {
        self.half_extents
    }

    /// The minimal point of the box. This is always equal to center - extents.
    #[must_use]
    pub fn min(&self) -> Vec2 {
        self.position - self.half_extents
    }

    /// The maximal point of the box. This is always equal to center + extents.
    #[must_use]
    pub fn max(&self) -> Vec2 {
        self.position + self.half_extents
    }

    /// The total size of the box. This is always twice as large as the extents.
    #[must_use]
    pub fn size(&self) -> Vec2 {
        self.half_extents * 2.0
    }

    /// Expand the bounds by increasing its size by amount along each side
    #[must_use]
    pub fn expand(&self, amount: Vec2) -> Self {
        let mut expanded_bounds = *self;
        expanded_bounds.half_extents += amount;
        expanded_bounds
    }
}
