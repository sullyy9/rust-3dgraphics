//! Implementations of arithmetic operations on vectors.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

use super::{Vector, Scalar, Matrix};

/// Vector + Vector = Vector
///
impl<const D: usize> Add<Vector<D>> for Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<const D: usize> Add<&Vector<D>> for Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: &Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<const D: usize> Add<Vector<D>> for &Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<const D: usize> Add<&Vector<D>> for &Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: &Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}

/// Vector += Vector
///
impl<const D: usize> AddAssign<Vector<D>> for Vector<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<const D: usize> AddAssign<&mut Vector<D>> for Vector<D> {
    fn add_assign(&mut self, rhs: &mut Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<const D: usize> AddAssign<Vector<D>> for &mut Vector<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<const D: usize> AddAssign<&mut Vector<D>> for &mut Vector<D> {
    fn add_assign(&mut self, rhs: &mut Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}

/// Scaler Arithmetic.
///
/// Vector * Scaler = Vector.
///
impl<const D: usize> Mul<Scalar> for Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<const D: usize> Mul<Scalar> for &Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}

/// Vector *= Scaler.
///
impl<const D: usize> MulAssign<Scalar> for Vector<D> {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.0.mul_assign(rhs);
    }
}
impl<const D: usize> MulAssign<Scalar> for &mut Vector<D> {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.0.mul_assign(rhs);
    }
}

/// Vector / Scaler = Vector.
///
impl<const D: usize> Div<Scalar> for Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: Scalar) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}
impl<const D: usize> Div<Scalar> for &Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: Scalar) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}

/// Vector /= Scaler.
///
impl<const D: usize> DivAssign<Scalar> for Vector<D> {
    fn div_assign(&mut self, rhs: Scalar) {
        self.0.div_assign(rhs);
    }
}
impl<const D: usize> DivAssign<Scalar> for &mut Vector<D> {
    fn div_assign(&mut self, rhs: Scalar) {
        self.0.div_assign(rhs);
    }
}

/// -Vector = Vector
///
impl<const D: usize> Neg for Vector<D> {
    type Output = Vector<D>;
    fn neg(self) -> Self::Output {
        self.map(|coord| coord.neg())
    }
}
impl<const D: usize> Neg for &Vector<D> {
    type Output = Vector<D>;
    fn neg(self) -> Self::Output {
        self.map(|coord| coord.neg())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Matrix = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for Vector<R> {
    type Output = Vector<C>;
    fn mul(self, rhs: Matrix<R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for &Vector<R> {
    type Output = Vector<C>;
    fn mul(self, rhs: Matrix<R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<const R: usize, const C: usize> Mul<&Matrix<R, C>> for Vector<R> {
    type Output = Vector<C>;
    fn mul(self, rhs: &Matrix<R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<const R: usize, const C: usize> Mul<&Matrix<R, C>> for &Vector<R> {
    type Output = Vector<C>;
    fn mul(self, rhs: &Matrix<R, C>) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point *= Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize> MulAssign<T> for Vector<R>
where
    T: AsRef<Matrix<R, R>>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.as_ref();
        self.0.mul_assign(rhs);
    }
}
impl<T, const R: usize> MulAssign<T> for &mut Vector<R>
where
    T: AsRef<Matrix<R, R>>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.as_ref();
        self.0.mul_assign(rhs);
    }
}