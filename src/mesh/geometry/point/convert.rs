//! Implementations of traits and methods for point type conversion.
//! 

use super::{Matrix, Point};

/// Point -> Point
/// 
impl<const D: usize> AsRef<Point<D>> for Point<D> {
    fn as_ref(&self) -> &Point<D> {
        self
    }
}
impl<const D: usize> AsMut<Point<D>> for Point<D> {
    fn as_mut(&mut self) -> &mut Point<D> {
        self
    }
}

impl<const D: usize> Point<D> {
    /// Promote a point to a higher dimentional point where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Point<ND> {
        let mut new_point = Point::default();
        let len = self.0[0].len();
        new_point.0[0][..len].clone_from_slice(&self.0[0]);
        new_point
    }

    /// Demote a point to a lower dimentional point.
    ///
    pub fn demote<const ND: usize>(&self) -> Point<ND> {
        let mut new_point = Point::default();
        let len = new_point.0[0].len();
        new_point.0[0].clone_from_slice(&self.0[0][..len]);
        new_point
    }
}

/// Point -> Matrix
/// 
impl<const D: usize> AsRef<Matrix<1, D>> for Point<D> {
    fn as_ref(&self) -> &Matrix<1, D> {
        &self.0
    }
}
impl<const D: usize> AsMut<Matrix<1, D>> for Point<D> {
    fn as_mut(&mut self) -> &mut Matrix<1, D> {
        &mut self.0
    }
}