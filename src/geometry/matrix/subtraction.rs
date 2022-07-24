//! Implementation of matrix subtraction.
//!

use std::ops::{Sub, SubAssign};

use super::{Matrix, MatrixElement};

////////////////////////////////////////////////////////////////////////////////
// Matrix - Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize, const C: usize> Sub<M> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: M) -> Self::Output {
        let mut mat = self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(*rhs));
        mat
    }
}
impl<T, M, const R: usize, const C: usize> Sub<M> for &Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: M) -> Self::Output {
        let mut mat = *self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(*rhs));
        mat
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix -= Matrix ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize, const C: usize> SubAssign<M> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    fn sub_assign(&mut self, rhs: M) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(*rhs));
    }
}
impl<T, M, const R: usize, const C: usize> SubAssign<M> for &mut Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, C>>,
{
    fn sub_assign(&mut self, rhs: M) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(*rhs));
    }
}
