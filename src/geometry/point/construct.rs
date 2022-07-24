//! Implementations of construction methods for points.
//!

use super::{Matrix, MatrixElement, Point};

impl<T, const D: usize> Default for Point<T, D>
where
    T: MatrixElement<T>,
{
    /// Construct a point at the origin.
    ///
    fn default() -> Self {
        Self(Matrix::new([[T::default(); D]]))
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: MatrixElement<T>,
{
    /// Construct a point from an array.
    ///
    pub fn new(coords: [T; D]) -> Self
    where
        T: Into<T>,
    {
        Self(Matrix::new([coords.map(|coord| coord)]))
    }
}

impl<T, const D: usize> From<Matrix<T, 1, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    /// Construct a point from a 1 row matrix.
    ///
    fn from(matrix: Matrix<T, 1, D>) -> Self {
        Point(matrix)
    }
}
