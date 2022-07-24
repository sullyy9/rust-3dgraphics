//! Implementations of point methods.
//!

use std::ops::{AddAssign, Sub};

use super::{Point, Vector, MatrixElement};

impl<T, const D: usize> Point<T, D>
where
    T: MatrixElement<T>,
{
    /// Return a Vector3D describing the transformation from the given point to this point.
    ///
    pub fn vector_from<P>(&self, point: &P) -> Vector<T, D>
    where
        P: AsRef<Point<T, D>>,
    {
        self.sub(point.as_ref())
    }

    /// Return a Vector3D describing the transformation from this point to the given point.
    ///
    pub fn vector_to<P>(&self, point: &P) -> Vector<T, D>
    where
        P: AsRef<Point<T, D>>,
    {
        point.as_ref().sub(self)
    }

    /// Translate a point by adding the given vector.
    ///
    pub fn translate(&mut self, vector: &Vector<T, D>) {
        self.add_assign(vector);
    }
}
