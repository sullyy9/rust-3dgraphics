//! Implementation of matrix addition.
//!

use std::ops::{Add, AddAssign};

use super::Matrix;

////////////////////////////////////////////////////////////////////////////////
// Matrix + Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> Add<T> for Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    type Output = Matrix<R, C>;

    fn add(self, rhs: T) -> Self::Output {
        let mut mat = self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        mat
    }
}
impl<T, const R: usize, const C: usize> Add<T> for &Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    type Output = Matrix<R, C>;

    fn add(self, rhs: T) -> Self::Output {
        let mut mat = *self;
        mat.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        mat
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix += Matrix ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> AddAssign<T> for Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    fn add_assign(&mut self, rhs: T) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<T, const R: usize, const C: usize> AddAssign<T> for &mut Matrix<R, C>
where
    T: AsRef<Matrix<R, C>>,
{
    fn add_assign(&mut self, rhs: T) {
        self.iter_mut()
            .zip(rhs.as_ref().iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
