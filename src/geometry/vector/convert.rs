//! Implementations of traits and methods for vector type conversion.
//!

use super::{Matrix, MatrixElement, Vector};

impl<T, const D: usize> AsRef<Vector<T, D>> for Vector<T, D> {
    fn as_ref(&self) -> &Vector<T, D> {
        self
    }
}
impl<T, const D: usize> AsMut<Vector<T, D>> for Vector<T, D> {
    fn as_mut(&mut self) -> &mut Vector<T, D> {
        self
    }
}

impl<T, const D: usize> Vector<T, D>
where
    T: MatrixElement<T>,
{
    /// Promote a vector to a higher dimentional vector where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Vector<T, ND> {
        let mut new_vector = Vector::default();
        let len = self.0[0].len();
        new_vector.0[0][..len].clone_from_slice(&self.0[0]);
        new_vector
    }

    /// Demote a vector to a lower dimentional vector.
    ///
    pub fn demote<const ND: usize>(&self) -> Vector<T, ND> {
        let mut new_vector = Vector::default();
        let len = new_vector.0[0].len();
        new_vector.0[0].clone_from_slice(&self.0[0][..len]);
        new_vector
    }
}

impl<T, const D: usize> AsRef<Matrix<T, 1, D>> for Vector<T, D> {
    fn as_ref(&self) -> &Matrix<T, 1, D> {
        &self.0
    }
}
impl<T, const D: usize> AsMut<Matrix<T, 1, D>> for Vector<T, D> {
    fn as_mut(&mut self) -> &mut Matrix<T, 1, D> {
        &mut self.0
    }
}