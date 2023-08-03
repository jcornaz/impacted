mod broad_phase_interop;
mod epa;
mod gjk;
mod math;
mod minkowski;
#[cfg(test)]
mod ray;
pub mod shapes;
mod transform;

use shapes::ShapeData;
pub use transform::Transform;

/// A collision shape
///
/// This is the entry point for collision detection.
///
/// See [crate](crate) level documentation for more info and examples.
#[derive(Debug, Clone)]
pub struct CollisionShape {
    transform: Transform,
    data: ShapeData,
}

impl<S: Into<ShapeData>> From<S> for CollisionShape {
    fn from(shape: S) -> Self {
        Self {
            transform: Transform::default(),
            data: shape.into(),
        }
    }
}

impl CollisionShape {
    /// Create a circle from its radius
    ///
    /// The origin is in the center of the circle
    #[inline]
    #[must_use]
    pub fn new_circle(radius: f32) -> Self {
        shapes::Circle::new(radius).into()
    }

    /// Create a rectangle from its width and height
    ///
    /// The origin is in the center of the rectangle
    #[inline]
    #[must_use]
    pub fn new_rectangle(width: f32, height: f32) -> Self {
        shapes::Rectangle::new(width, height).into()
    }

    /// Create a segment from two points
    #[inline]
    #[must_use]
    pub fn new_segment(p1: impl Into<[f32; 2]>, p2: impl Into<[f32; 2]>) -> Self {
        shapes::Segment::new(p1, p2).into()
    }

    /// Set the transform (translation, rotation and scale)
    ///
    /// This is equivalent to [`set_transform`](Self::set_transform), but in a builder style,
    /// useful to set the transform directly at creation
    #[inline]
    #[must_use]
    pub fn with_transform(mut self, transform: impl Into<Transform>) -> Self {
        self.set_transform(transform);
        self
    }

    /// Set the transform (translation, rotation and scale)
    #[inline]
    pub fn set_transform(&mut self, transform: impl Into<Transform>) {
        self.transform = transform.into();
    }

    /// Returns true if the two convex shapes geometries are overlapping
    #[must_use]
    pub fn is_collided_with(&self, other: &Self) -> bool {
        let difference = minkowski::Difference {
            shape1: self,
            shape2: other,
        };
        let initial_axis = other.transform.position() - self.transform.position();
        gjk::find_simplex_enclosing_origin(&difference, initial_axis).is_some()
    }

    /// Returns contact data with the other shape if they collide. Returns `None` if they don't collide.
    ///
    /// The normal of the contact data is pointing toward this shape.
    /// In other words, ff this shape is moved by `contact.normal * contact.penetration`
    /// the two shapes will no longer be inter-penetrating.
    #[must_use]
    pub fn contact_with(&self, other: &Self) -> Option<Contact> {
        let difference = minkowski::Difference {
            shape1: self,
            shape2: other,
        };
        let initial_axis = other.transform.position() - self.transform.position();
        let simplex = gjk::find_simplex_enclosing_origin(&difference, initial_axis)?;
        let Contact {
            normal,
            penetration,
        } = epa::generate_contact(&difference, simplex);
        Some(Contact::<f32, [f32; 2]> {
            normal: normal.into(),
            penetration,
        })
    }

    /// Returns the shape data of the collider
    #[must_use]
    pub fn shape_data(&self) -> &ShapeData {
        &self.data
    }
}

/// Contact data between two shapes
///
/// See [`CollisionShape::contact_with`]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Contact<S = f32, V = [S; 2]> {
    /// Contact normal
    ///
    /// This is the direction on which the first shape should be moved to resolve inter-penetration
    /// This is also on that direction that impulse should be applied to the first shape to resolve velocities
    pub normal: V,
    /// Penetration
    ///
    /// This is "how much" the two shapes are inter-penetrating
    pub penetration: S,
}

trait Support<V> {
    /// Returns the farthest point of the shape in the given direction.
    ///
    /// More formaly: For a direction `v` return the point `p` of the shape that maximize the dot product `p . v`
    ///
    /// If many points are equaly far in the given direction (have the same dot product `p . v`),
    /// then one of the is choosen arbitrarily.
    ///
    /// Note the direction may not be normalized, and may have a magnitude of zero.
    fn support(&self, direction: V) -> V;
}
