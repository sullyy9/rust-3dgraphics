//! Implementations of construction methods for points.
//!

use super::{Matrix, Point};

impl<const D: usize> Default for Point<D> {
    /// Construct a point at the origin.
    ///
    fn default() -> Self {
        Self(Matrix::new([[0.0; D]]))
    }
}

impl<const D: usize> Point<D> {
    /// Construct a point from an array.
    ///
    pub fn new<T>(coords: [T; D]) -> Self
    where
        T: Into<f64>,
    {
        Self(Matrix::new([coords.map(|coord| coord.into())]))
    }
}


impl<const D: usize> From<Matrix<1, D>> for Point<D> {
    /// Construct a point from a 1 row matrix.
    ///
    fn from(matrix: Matrix<1, D>) -> Self {
        Point(matrix)
    }
}
