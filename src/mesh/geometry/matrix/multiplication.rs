//! Implementation of matrix multiplication.
//!

use std::ops::{Mul, MulAssign};

use super::{Matrix, Scalar};

////////////////////////////////////////////////////////////////////////////////
// Matrix * Scalar /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const R: usize, const C: usize> Mul<Scalar> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.map(|lhs| lhs.mul(rhs.0))
    }
}
impl<const R: usize, const C: usize> Mul<Scalar> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self.map(|lhs| lhs.mul(rhs.0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix *= Scalar ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const R: usize, const C: usize> MulAssign<Scalar> for Matrix<R, C> {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.for_each(|lhs| lhs.mul_assign(rhs.0));
    }
}
impl<const R: usize, const C: usize> MulAssign<Scalar> for &mut Matrix<R, C> {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.for_each(|lhs| lhs.mul_assign(rhs.0));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix * Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////
// Matrix *= Matrix ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const N: usize> MulAssign<T> for Matrix<N, N>
where
    T: AsRef<Matrix<N, N>>,
{
    fn mul_assign(&mut self, rhs: T) {
        let mut mat = Matrix::default();
        let rhs = rhs.as_ref();
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
