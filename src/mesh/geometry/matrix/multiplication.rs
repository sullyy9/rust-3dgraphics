//! Implementation of matrix multiplication.
//!

use std::ops::{Mul, MulAssign};

use super::Matrix;

/// Matrix * Scaler multiplication.
///
impl<T: Into<f64>, const R: usize, const C: usize> Mul<T> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|lhs| lhs.mul(rhs))
    }
}
impl<T: Into<f64>, const R: usize, const C: usize> Mul<T> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|lhs| lhs.mul(rhs))
    }
}

impl<T: Into<f64>, const R: usize, const C: usize> MulAssign<T> for Matrix<R, C> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.for_each(|lhs| lhs.mul_assign(rhs));
    }
}

/// Matrix * Matrix multiplication.
///
macro_rules! mul_impl {
    ({$lhs_t:ty} * {$rhs_t:ty}) => {
        impl<const N: usize, const M: usize, const P: usize> Mul<$rhs_t> for $lhs_t {
            type Output = Matrix<M, P>;

            fn mul(self, rhs: $rhs_t) -> Self::Output {
                let mut mat = Self::Output::default();

                for m in 0..M {
                    for p in 0..P {
                        mat[m][p] = (0..N).fold(0.0, |sum, n| sum + (self[m][n] * rhs[n][p]));
                    }
                }
                mat
            }
        }
    };
}

mul_impl! {{Matrix<M, N>} * {Matrix<N, P>}}
mul_impl! {{Matrix<M, N>} * {&Matrix<N, P>}}

mul_impl! {{&Matrix<M, N>} * {Matrix<N, P>}}
mul_impl! {{&Matrix<M, N>} * {&Matrix<N, P>}}

/// Matrix *= Matrix multiplication.
///
impl<const N: usize> MulAssign<Matrix<N, N>> for Matrix<N, N> {
    fn mul_assign(&mut self, rhs: Matrix<N, N>) {
        let mut mat = Matrix::default();

        for r in 0..N {
            for c in 0..N {
                mat[r][c] = (0..N).fold(0.0, |sum, n| sum + (self[r][n] * rhs[n][c]));
            }
        }
        *self = mat;
    }
}
impl<const N: usize> MulAssign<Matrix<N, N>> for &mut Matrix<N, N> {
    fn mul_assign(&mut self, rhs: Matrix<N, N>) {
        let mut mat = Matrix::default();

        for r in 0..N {
            for c in 0..N {
                mat[r][c] = (0..N).fold(0.0, |sum, n| sum + (self[r][n] * rhs[n][c]));
            }
        }
        **self = mat;
    }
}
impl<const N: usize> MulAssign<&mut Matrix<N, N>> for Matrix<N, N> {
    fn mul_assign(&mut self, rhs: &mut Matrix<N, N>) {
        let mut mat = Matrix::default();

        for r in 0..N {
            for c in 0..N {
                mat[r][c] = (0..N).fold(0.0, |sum, n| sum + (self[r][n] * rhs[n][c]));
            }
        }
        *self = mat;
    }
}
impl<const N: usize> MulAssign<&mut Matrix<N, N>> for &mut Matrix<N, N> {
    fn mul_assign(&mut self, rhs: &mut Matrix<N, N>) {
        let mut mat = Matrix::default();

        for r in 0..N {
            for c in 0..N {
                mat[r][c] = (0..N).fold(0.0, |sum, n| sum + (self[r][n] * rhs[n][c]));
            }
        }
        **self = mat;
    }
}
