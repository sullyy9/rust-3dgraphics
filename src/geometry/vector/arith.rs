//! Implementations of arithmetic operations on vectors.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

use super::{Matrix, MatrixElement, Scalar, Vector};

/// Vector + Vector = Vector
///
impl<T, const D: usize> Add<Vector<T, D>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn add(self, rhs: Vector<T, D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<T, const D: usize> Add<&Vector<T, D>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn add(self, rhs: &Vector<T, D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<T, const D: usize> Add<Vector<T, D>> for &Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn add(self, rhs: Vector<T, D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<T, const D: usize> Add<&Vector<T, D>> for &Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn add(self, rhs: &Vector<T, D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}

/// Vector += Vector
///
impl<T, const D: usize> AddAssign<Vector<T, D>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn add_assign(&mut self, rhs: Vector<T, D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<T, const D: usize> AddAssign<&mut Vector<T, D>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn add_assign(&mut self, rhs: &mut Vector<T, D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<T, const D: usize> AddAssign<Vector<T, D>> for &mut Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn add_assign(&mut self, rhs: Vector<T, D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<T, const D: usize> AddAssign<&mut Vector<T, D>> for &mut Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn add_assign(&mut self, rhs: &mut Vector<T, D>) {
        self.0.add_assign(rhs.0);
    }
}

/// Scaler Arithmetic.
///
/// Vector * Scaler = Vector.
///
impl<T, const D: usize> Mul<Scalar<T>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<T, const D: usize> Mul<Scalar<T>> for &Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}

/// Vector *= Scaler.
///
impl<T, const D: usize> MulAssign<Scalar<T>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn mul_assign(&mut self, rhs: Scalar<T>) {
        self.0.mul_assign(rhs);
    }
}
impl<T, const D: usize> MulAssign<Scalar<T>> for &mut Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn mul_assign(&mut self, rhs: Scalar<T>) {
        self.0.mul_assign(rhs);
    }
}

/// Vector / Scaler = Vector.
///
impl<T, const D: usize> Div<Scalar<T>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}
impl<T, const D: usize> Div<Scalar<T>> for &Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, D>;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}

/// Vector /= Scaler.
///
impl<T, const D: usize> DivAssign<Scalar<T>> for Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn div_assign(&mut self, rhs: Scalar<T>) {
        self.0.div_assign(rhs);
    }
}
impl<T, const D: usize> DivAssign<Scalar<T>> for &mut Vector<T, D>
where
    T: MatrixElement<T>,
{
    fn div_assign(&mut self, rhs: Scalar<T>) {
        self.0.div_assign(rhs);
    }
}

/// -Vector = Vector
///
impl<T, const D: usize> Neg for Vector<T, D>
where
    T: MatrixElement<T> + Neg<Output = T>,
{
    type Output = Vector<T, D>;
    fn neg(self) -> Self::Output {
        self.map(|coord| coord.neg())
    }
}
impl<T, const D: usize> Neg for &Vector<T, D>
where
    T: MatrixElement<T> + Neg<Output = T>,
{
    type Output = Vector<T, D>;
    fn neg(self) -> Self::Output {
        self.map(|coord| coord.neg())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Matrix = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> Mul<Matrix<T, R, C>> for Vector<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, C>;
    fn mul(self, rhs: Matrix<T, R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<T, const R: usize, const C: usize> Mul<Matrix<T, R, C>> for &Vector<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, C>;
    fn mul(self, rhs: Matrix<T, R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<T, const R: usize, const C: usize> Mul<&Matrix<T, R, C>> for Vector<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, C>;
    fn mul(self, rhs: &Matrix<T, R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<T, const R: usize, const C: usize> Mul<&Matrix<T, R, C>> for &Vector<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Vector<T, C>;
    fn mul(self, rhs: &Matrix<T, R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point *= Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize> MulAssign<M> for Vector<T, R>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, R>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let rhs = rhs.as_ref();
        self.0.mul_assign(rhs);
    }
}
impl<T, M, const R: usize> MulAssign<M> for &mut Vector<T, R>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, R>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let rhs = rhs.as_ref();
        self.0.mul_assign(rhs);
    }
}
