//! Implementation of matrix multiplication.
//!

use std::ops::{Div, DivAssign};

use super::{Matrix, Scalar, MatrixElement};

/// Matrix / Scaler multiplication.
///
impl<T, const R: usize, const C: usize> Div<Scalar<T>> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    type Output = Matrix<T, R, C>;

    fn div(self, rhs: Scalar<T>) -> Self::Output {
        self.map(|lhs| lhs.div(rhs.0))
    }
}
impl<T, const R: usize, const C: usize> Div<Scalar<T>> for &Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    type Output = Matrix<T, R, C>;

    fn div(self, rhs: Scalar<T>) -> Self::Output {
        self.map(|lhs| lhs.div(rhs.0))
    }
}
impl<T, const R: usize, const C: usize> DivAssign<Scalar<T>> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    fn div_assign(&mut self, rhs: Scalar<T>) {
        self.for_each(|lhs| lhs.div_assign(rhs.0));
    }
}
