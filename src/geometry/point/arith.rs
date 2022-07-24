//! Implementations of arithmetic operations on points.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use super::{Matrix, MatrixElement, Point, Scalar, Vector};

////////////////////////////////////////////////////////////////////////////////
// Point + Vector = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, V, const N: usize> Add<V> for Point<T, N>
where
    T: MatrixElement<T>,
    V: AsRef<Vector<T, N>>,
{
    type Output = Point<T, N>;
    fn add(self, rhs: V) -> Self::Output {
        Point(self.0.add(&rhs.as_ref().0))
    }
}
impl<T, V, const N: usize> Add<V> for &Point<T, N>
where
    T: MatrixElement<T>,
    V: AsRef<Vector<T, N>>,
{
    type Output = Point<T, N>;
    fn add(self, rhs: V) -> Self::Output {
        Point(self.0.add(&rhs.as_ref().0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point += Vector /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, V, const N: usize> AddAssign<V> for Point<T, N>
where
    T: MatrixElement<T>,
    V: AsRef<Vector<T, N>>,
{
    fn add_assign(&mut self, rhs: V) {
        self.0.add_assign(&rhs.as_ref().0);
    }
}
impl<T, V, const N: usize> AddAssign<V> for &mut Point<T, N>
where
    T: MatrixElement<T>,
    V: AsRef<Vector<T, N>>,
{
    fn add_assign(&mut self, rhs: V) {
        self.0.add_assign(&rhs.as_ref().0);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point - Point = Vector //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, P, const N: usize> Sub<P> for Point<T, N>
where
    T: MatrixElement<T>,
    P: AsRef<Point<T, N>>,
{
    type Output = Vector<T, N>;
    fn sub(self, rhs: P) -> Self::Output {
        Vector(self.0.sub(&rhs.as_ref().0))
    }
}
impl<T, P, const N: usize> Sub<P> for &Point<T, N>
where
    T: MatrixElement<T>,
    P: AsRef<Point<T, N>>,
{
    type Output = Vector<T, N>;
    fn sub(self, rhs: P) -> Self::Output {
        Vector(self.0.sub(&rhs.as_ref().0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Scaler = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const D: usize> Mul<Scalar<T>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, D>;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<T, const D: usize> Mul<Scalar<T>> for &Point<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, D>;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point *= Scaler /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const D: usize> MulAssign<Scalar<T>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn mul_assign(&mut self, rhs: Scalar<T>) {
        self.0.mul_assign(rhs);
    }
}
impl<T, const D: usize> MulAssign<Scalar<T>> for &mut Point<T, D>
where
    T: MatrixElement<T>,
{
    fn mul_assign(&mut self, rhs: Scalar<T>) {
        self.0.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point / Scaler = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const D: usize> Div<Scalar<T>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, D>;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Point(self.0.div(rhs))
    }
}
impl<T, const D: usize> Div<Scalar<T>> for &Point<T, D>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, D>;
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Point(self.0.div(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point /= Scaler /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const D: usize> DivAssign<Scalar<T>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn div_assign(&mut self, rhs: Scalar<T>) {
        self.0.div_assign(rhs);
    }
}
impl<T, const D: usize> DivAssign<Scalar<T>> for &mut Point<T, D>
where
    T: MatrixElement<T>,
{
    fn div_assign(&mut self, rhs: Scalar<T>) {
        self.0.div_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Matrix = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const R: usize, const C: usize> Mul<Matrix<T, R, C>> for Point<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, C>;
    fn mul(self, rhs: Matrix<T, R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<T, const R: usize, const C: usize> Mul<Matrix<T, R, C>> for &Point<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, C>;
    fn mul(self, rhs: Matrix<T, R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<T, const R: usize, const C: usize> Mul<&Matrix<T, R, C>> for Point<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, C>;
    fn mul(self, rhs: &Matrix<T, R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<T, const R: usize, const C: usize> Mul<&Matrix<T, R, C>> for &Point<T, R>
where
    T: MatrixElement<T>,
{
    type Output = Point<T, C>;
    fn mul(self, rhs: &Matrix<T, R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point *= Matrix /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, M, const R: usize> MulAssign<M> for Point<T, R>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, R>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let rhs = rhs.as_ref();
        self.0.mul_assign(rhs);
    }
}
impl<T, M, const R: usize> MulAssign<M> for &mut Point<T, R>
where
    T: MatrixElement<T>,
    M: AsRef<Matrix<T, R, R>>,
{
    fn mul_assign(&mut self, rhs: M) {
        let rhs = rhs.as_ref();
        self.0.mul_assign(rhs);
    }
}
