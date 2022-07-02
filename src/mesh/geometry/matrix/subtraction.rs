//! Implementation of matrix subtraction.
//!

use std::ops::{Sub, SubAssign};

use super::Matrix;

////////////////////////////////////////////////////////////////////////////////
// Matrix - Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> Sub<T> for Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    type Output = Matrix<R, C>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut mat = self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        mat
    }
}
impl<T, const R: usize, const C: usize> Sub<T> for &Matrix<R, C> 
where
    T: AsRef<Matrix<R, C>>,
{
    type Output = Matrix<R, C>;

    fn sub(self, rhs: T) -> Self::Output {
        let mut mat = *self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        mat
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix -= Matrix ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> SubAssign<T> for Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
    }
}
impl<T, const R: usize, const C: usize> SubAssign<T> for &mut Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
    }
}