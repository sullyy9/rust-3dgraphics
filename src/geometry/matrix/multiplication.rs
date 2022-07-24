//! Implementation of matrix multiplication.
//!

use std::ops::{Mul, MulAssign};

use super::{Matrix, MatrixElement, Scalar};

////////////////////////////////////////////////////////////////////////////////
// Matrix * Scalar /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> Mul<Scalar<T>> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        self.map(|lhs| lhs.mul(rhs.0))
    }
}
impl<T, const R: usize, const C: usize> Mul<Scalar<T>> for &Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        self.map(|lhs| lhs.mul(rhs.0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix *= Scalar ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> MulAssign<Scalar<T>> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    fn mul_assign(&mut self, rhs: Scalar<T>) {
        self.for_each(|lhs| lhs.mul_assign(rhs.0));
    }
}
impl<T, const R: usize, const C: usize> MulAssign<Scalar<T>> for &mut Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    fn mul_assign(&mut self, rhs: Scalar<T>) {
        self.for_each(|lhs| lhs.mul_assign(rhs.0));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix * Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
macro_rules! mul_impl {
    ({$lhs_t:ty} * {$rhs_t:ty}) => {
        impl<T, const N: usize, const M: usize, const P: usize> Mul<$rhs_t> for $lhs_t
        where
            T: MatrixElement<T>,
        {
            type Output = Matrix<T, M, P>;

            fn mul(self, rhs: $rhs_t) -> Self::Output {
                let mut mat = Self::Output::default();

                for m in 0..M {
                    for p in 0..P {
                        mat[m][p] =
                            (0..N).fold(T::default(), |sum, n| sum + (self[m][n] * rhs[n][p]));
                    }
                }
                mat
            }
        }
    };
}

mul_impl! {{Matrix<T, M, N>} * {Matrix<T, N, P>}}
mul_impl! {{Matrix<T, M, N>} * {&Matrix<T, N, P>}}

mul_impl! {{&Matrix<T, M, N>} * {Matrix<T, N, P>}}
mul_impl! {{&Matrix<T, M, N>} * {&Matrix<T, N, P>}}

////////////////////////////////////////////////////////////////////////////////
// Matrix *= Matrix ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize, const C: usize> MulAssign<M> for Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, C, C>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let mut mat = Matrix::default();
        let rhs = rhs.as_ref();
        for r in 0..R {
            for c in 0..C {
                mat[r][c] = (0..C).fold(T::default(), |sum, n| sum + (self[r][n] * rhs[n][c]));
            }
        }
        *self = mat;
    }
}
impl<T, M, const R: usize, const C: usize> MulAssign<M> for &mut Matrix<T, R, C>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, C, C>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let mut mat = Matrix::default();
        let rhs = rhs.as_ref();
        for r in 0..R {
            for c in 0..C {
                mat[r][c] = (0..C).fold(T::default(), |sum, n| sum + (self[r][n] * rhs[n][c]));
            }
        }
        **self = mat;
    }
}
