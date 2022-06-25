//! Implementation of matrix multiplication.
//!

use std::ops::{Div, DivAssign};

use super::Matrix;

/// Matrix / Scaler multiplication.
/// 
impl<T: Into<f64>, const R: usize, const C: usize> Div<T> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|lhs| lhs.div(rhs))
    }
}
impl<T: Into<f64>, const R: usize, const C: usize> Div<T> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|lhs| lhs.div(rhs))
    }
}
impl<T: Into<f64>, const R: usize, const C: usize> DivAssign<T> for Matrix<R, C> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.for_each(|lhs| lhs.div_assign(rhs));
    }
}