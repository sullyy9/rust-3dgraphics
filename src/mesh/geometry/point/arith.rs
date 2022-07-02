//! Implementations of arithmetic operations on points.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use super::{Matrix, Point, Scalar, Vector};

////////////////////////////////////////////////////////////////////////////////
// Point + Vector = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const N: usize> Add<T> for Point<N>
where
    T: AsRef<Vector<N>>,
{
    type Output = Point<N>;
    fn add(self, rhs: T) -> Self::Output {
        Point(self.0.add(&rhs.as_ref().0))
    }
}
impl<T, const N: usize> Add<T> for &Point<N>
where
    T: AsRef<Vector<N>>,
{
    type Output = Point<N>;
    fn add(self, rhs: T) -> Self::Output {
        Point(self.0.add(&rhs.as_ref().0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point += Vector /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const N: usize> AddAssign<T> for Point<N>
where
    T: AsRef<Vector<N>>,
{
    fn add_assign(&mut self, rhs: T) {
        self.0.add_assign(&rhs.as_ref().0);
    }
}
impl<T, const N: usize> AddAssign<T> for &mut Point<N>
where
    T: AsRef<Vector<N>>,
{
    fn add_assign(&mut self, rhs: T) {
        self.0.add_assign(&rhs.as_ref().0);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point - Point = Vector //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T, const N: usize> Sub<T> for Point<N>
where
    T: AsRef<Point<N>>,
{
    type Output = Vector<N>;
    fn sub(self, rhs: T) -> Self::Output {
        Vector(self.0.sub(&rhs.as_ref().0))
    }
}
impl<T, const N: usize> Sub<T> for &Point<N>
where
    T: AsRef<Point<N>>,
{
    type Output = Vector<N>;
    fn sub(self, rhs: T) -> Self::Output {
        Vector(self.0.sub(&rhs.as_ref().0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Scaler = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> Mul<Scalar> for Point<D> {
    type Output = Point<D>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<const D: usize> Mul<Scalar> for &Point<D> {
    type Output = Point<D>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point *= Scaler /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> MulAssign<Scalar> for Point<D> {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.0.mul_assign(rhs);
    }
}
impl<const D: usize> MulAssign<Scalar> for &mut Point<D> {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.0.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point / Scaler = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> Div<Scalar> for Point<D> {
    type Output = Point<D>;
    fn div(self, rhs: Scalar) -> Self::Output {
        Point(self.0.div(rhs))
    }
}
impl<const D: usize> Div<Scalar> for &Point<D> {
    type Output = Point<D>;
    fn div(self, rhs: Scalar) -> Self::Output {
        Point(self.0.div(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point /= Scaler /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> DivAssign<Scalar> for Point<D> {
    fn div_assign(&mut self, rhs: Scalar) {
        self.0.div_assign(rhs);
    }
}
impl<const D: usize> DivAssign<Scalar> for &mut Point<D> {
    fn div_assign(&mut self, rhs: Scalar) {
        self.0.div_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Matrix = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for Point<R> {
    type Output = Point<C>;
    fn mul(self, rhs: Matrix<R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for &Point<R> {
    type Output = Point<C>;
    fn mul(self, rhs: Matrix<R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<const R: usize, const C: usize> Mul<&Matrix<R, C>> for Point<R> {
    type Output = Point<C>;
    fn mul(self, rhs: &Matrix<R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<const R: usize, const C: usize> Mul<&Matrix<R, C>> for &Point<R> {
    type Output = Point<C>;
    fn mul(self, rhs: &Matrix<R, C>) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
