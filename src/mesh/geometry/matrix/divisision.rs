//! Implementation of matrix multiplication.
//!

use std::ops::{Div, DivAssign};

use super::Matrix;

/// Scalar * Matrix multiplication.
/// 
impl<T: Into<f64>, const R: usize, const C: usize> Div<T> for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|a| a / rhs)
    }
}

impl<T: Into<f64>,const R: usize, const C: usize> Div<T> for &Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|a| a / rhs)
    }
}

impl<T: Into<f64>,const R: usize, const C: usize> Div<T> for &mut Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        self.map(|a| a / rhs)
    }
}

impl<T: Into<f64>, const R: usize, const C: usize> DivAssign<T> for Matrix<R, C> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.for_each(|a| a.div_assign(rhs));
    }
}

impl<T: Into<f64>,const R: usize, const C: usize> DivAssign<T> for &mut Matrix<R, C> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.for_each(|a| a.div_assign(rhs));
    }
}

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