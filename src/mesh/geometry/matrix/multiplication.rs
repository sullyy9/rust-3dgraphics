//! Implementation of matrix multiplication.
//!

use std::ops::{Mul, MulAssign};

use super::Matrix;

/// Matrix * Scaler multiplication.
///
macro_rules! mul_scaler_impl {
    ({$lhs_t:ty} * {T}) => {
        impl<T: Into<f64>, const R: usize, const C: usize> Mul<T> for $lhs_t {
            type Output = Matrix<R, C>;

            fn mul(self, rhs: T) -> Self::Output {
                let rhs = rhs.into();
                self.map(|lhs| lhs.mul(rhs))
            }
        }
    };
    ({$lhs_t:ty} *= T) => {
        impl<T: Into<f64>, const R: usize, const C: usize> MulAssign<T> for $lhs_t {
            fn mul_assign(&mut self, rhs: T) {
                let rhs = rhs.into();
                self.iter_mut().for_each(|lhs| lhs.mul_assign(rhs));
            }
        }
    };
}

mul_scaler_impl! {{Matrix<R,C>} * {T}}
mul_scaler_impl! {{&Matrix<R,C>} * {T}}
mul_scaler_impl! {{Matrix<R,C>} *= T}
mul_scaler_impl! {{&mut Matrix<R,C>} *= T}

/// Matrix * Matrix multiplication.
///
impl<const R: usize, const C: usize> Mul for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> Mul<&Self> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: &Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> Mul for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: &Matrix<R, C>) -> Self::Output {
        todo!()
    }
}

impl<const R: usize, const C: usize> MulAssign for Matrix<R, C> {
    fn mul_assign(&mut self, rhs: Matrix<R, C>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> MulAssign<&Self> for Matrix<R, C> {
    fn mul_assign(&mut self, rhs: &Matrix<R, C>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> MulAssign<Matrix<R, C>> for &Matrix<R, C> {
    fn mul_assign(&mut self, rhs: Matrix<R, C>) {
        todo!()
    }
}

impl<const R: usize, const C: usize> MulAssign for &Matrix<R, C> {
    fn mul_assign(&mut self, rhs: &Matrix<R, C>) {
        todo!()
    }
}
