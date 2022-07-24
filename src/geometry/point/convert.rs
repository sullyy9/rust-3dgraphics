//! Implementations of traits and methods for point type conversion.
//!

use super::{Matrix, MatrixElement, Point};

/// Point -> Point
///
impl<T, const D: usize> AsRef<Point<T, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_ref(&self) -> &Point<T, D> {
        self
    }
}
impl<T, const D: usize> AsMut<Point<T, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_mut(&mut self) -> &mut Point<T, D> {
        self
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: MatrixElement<T>,
{
    /// Promote a point to a higher dimentional point where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Point<T, ND> {
        let mut new_point = Point::default();
        let len = self.0[0].len();
        new_point.0[0][..len].clone_from_slice(&self.0[0]);
        new_point
    }

    /// Demote a point to a lower dimentional point.
    ///
    pub fn demote<const ND: usize>(&self) -> Point<T, ND> {
        let mut new_point = Point::default();
        let len = new_point.0[0].len();
        new_point.0[0].clone_from_slice(&self.0[0][..len]);
        new_point
    }
}

/// Point -> Matrix
///
impl<T, const D: usize> AsRef<Matrix<T, 1, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_ref(&self) -> &Matrix<T, 1, D> {
        &self.0
    }
}
impl<T, const D: usize> AsMut<Matrix<T, 1, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_mut(&mut self) -> &mut Matrix<T, 1, D> {
        &mut self.0
    }
}