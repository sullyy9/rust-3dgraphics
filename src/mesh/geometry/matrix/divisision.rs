//! Implementation of matrix multiplication.
//!

use std::ops::{Div, DivAssign};

use super::{Matrix, Scalar};

/// Matrix / Scaler multiplication.
/// 
impl<const R: usize, const C: usize> Div<Scalar> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: Scalar) -> Self::Output {
        self.map(|lhs| lhs.div(rhs.0))
    }
}
impl<const R: usize, const C: usize> Div<Scalar> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: Scalar) -> Self::Output {
        self.map(|lhs| lhs.div(rhs.0))
    }
}
impl<const R: usize, const C: usize> DivAssign<Scalar> for Matrix<R, C> {
    fn div_assign(&mut self, rhs: Scalar) {
        self.for_each(|lhs| lhs.div_assign(rhs.0));
    }
}