//! Implementation of matrix multiplication.
//!

use std::ops::{Mul, MulAssign};

use super::Matrix;


/// Scalar * Matrix multiplication.
/// 
impl<T: Into<f64>, const R: usize, const C: usize> Mul<T> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|a| a * rhs)
    }
}

impl<T: Into<f64>,const R: usize, const C: usize> Mul<T> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|a| a * rhs)
    }
}

// impl<T: Into<f64>,const R: usize, const C: usize> Mul<T> for &mut Matrix<R, C> {
//     type Output = Matrix<R, C>;

//     fn mul(self, rhs: T) -> Self::Output {
//         let rhs = rhs.into();
//         self.map(|a| a * rhs)
//     }
// }

impl<T: Into<f64>, const R: usize, const C: usize> MulAssign<T> for Matrix<R, C> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.for_each(|a| a.mul_assign(rhs));
    }
}

impl<T: Into<f64>,const R: usize, const C: usize> MulAssign<T> for &mut Matrix<R, C> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.for_each(|a| a.mul_assign(rhs));
    }
}

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