//! Implementation of matrix addition.
//!

use std::ops::{Add, AddAssign};

use super::{mat::MatrixElement, Matrix};

////////////////////////////////////////////////////////////////////////////////
// Matrix + Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize, const C: usize> Add<M> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: M) -> Self::Output {
        let mut mat = self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(*rhs));
        mat
    }
}
impl<T, M, const R: usize, const C: usize> Add<M> for &Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: M) -> Self::Output {
        let mut mat = *self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(*rhs));
        mat
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix += Matrix ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize, const C: usize> AddAssign<M> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    fn add_assign(&mut self, rhs: M) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(*rhs));
    }
}
impl<T, M, const R: usize, const C: usize> AddAssign<M> for &mut Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    fn add_assign(&mut self, rhs: M) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(*rhs));
    }
}
