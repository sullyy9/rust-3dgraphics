//! Implementations of point methods.
//! 

use std::ops::{Sub, AddAssign};

use super::{Point, Vector};

impl<const D: usize> Point<D> {
    /// Return a Vector3D describing the transformation from the given point to this point.
    ///
    pub fn vector_from<T>(&self, point: &T) -> Vector<D>
    where
        T: AsRef<Point<D>>,
    {
        self.sub(point.as_ref())
    }

    /// Return a Vector3D describing the transformation from this point to the given point.
    ///
    pub fn vector_to<T>(&self, point: &T) -> Vector<D>
    where
        T: AsRef<Point<D>>,
    {
        point.as_ref().sub(self)
    }

    /// Translate a point by adding the given vector.
    ///
    pub fn translate(&mut self, vector: &Vector<D>) {
        self.add_assign(vector);
    }
}