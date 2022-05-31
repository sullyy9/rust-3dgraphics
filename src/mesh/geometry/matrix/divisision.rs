//! Implementation of matrix multiplication.
//!

use std::ops::{Div, DivAssign};

use super::Matrix;

/// Scalar * Matrix multiplication.
/// 
macro_rules! mul_scaler_impl {
    ({$lhs_t:ty} / {T}) => {
        impl<T: Into<f64>, const R: usize, const C: usize> Div<T> for $lhs_t {
            type Output = Matrix<R, C>;

            fn div(self, rhs: T) -> Self::Output {
                let rhs = rhs.into();
                self.map(|lhs| lhs.div(rhs))
            }
        }
    };
    ({$lhs_t:ty} /= T) => {
        impl<T: Into<f64>, const R: usize, const C: usize> DivAssign<T> for $lhs_t {
            fn div_assign(&mut self, rhs: T) {
                let rhs = rhs.into();
                self.iter_mut().for_each(|lhs| lhs.div_assign(rhs));
            }
        }
    };
}

mul_scaler_impl! {{Matrix<R,C>} / {T}}
mul_scaler_impl! {{&Matrix<R,C>} / {T}}
mul_scaler_impl! {{Matrix<R,C>} /= T}
mul_scaler_impl! {{&mut Matrix<R,C>} /= T}

/// Matrix * Matrix multiplication.
/// 
impl<const R: usize, const C: usize> Div for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> Div<&Self> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: &Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> Div<Matrix<R, C>> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> Div for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: &Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> DivAssign for Matrix<R, C> {
    fn div_assign(&mut self, rhs: Matrix<R, C>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> DivAssign<&Self> for Matrix<R, C> {
    fn div_assign(&mut self, rhs: &Matrix<R, C>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> DivAssign<Matrix<R, C>> for &Matrix<R, C> {
    fn div_assign(&mut self, rhs: Matrix<R, C>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> DivAssign for &Matrix<R, C> {
    fn div_assign(&mut self, rhs: &Matrix<R, C>) {
        todo!()
    }
}