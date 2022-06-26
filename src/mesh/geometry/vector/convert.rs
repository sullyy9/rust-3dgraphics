//! Implementations of traits and methods for vector type conversion.
//! 

use super::{Vector, Matrix};

impl<const D: usize> AsRef<Vector<D>> for Vector<D> {
    fn as_ref(&self) -> &Vector<D> {
        self
    }
}
impl<const D: usize> AsMut<Vector<D>> for Vector<D> {
    fn as_mut(&mut self) -> &mut Vector<D> {
        self
    }
}

impl<const D: usize> Vector<D> {
    /// Promote a vector to a higher dimentional vector where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Vector<ND> {
        let mut new_vector = Vector::default();
        let len = self.0[0].len();
        new_vector.0[0][..len].clone_from_slice(&self.0[0]);
        new_vector
    }

    /// Demote a vector to a lower dimentional vector.
    ///
    pub fn demote<const ND: usize>(&self) -> Vector<ND> {
        let mut new_vector = Vector::default();
        let len = new_vector.0[0].len();
        new_vector.0[0].clone_from_slice(&self.0[0][..len]);
        new_vector
    }
}

impl<const D: usize> AsRef<Matrix<1, D>> for Vector<D> {
    fn as_ref(&self) -> &Matrix<1, D> {
        &self.0
    }
}
impl<const D: usize> AsMut<Matrix<1, D>> for Vector<D> {
    fn as_mut(&mut self) -> &mut Matrix<1, D> {
        &mut self.0
    }
}

