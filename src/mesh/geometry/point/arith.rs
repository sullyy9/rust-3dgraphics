//! Implementations of arithmetic operations on points.
//! 

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use super::{Matrix, Point, Vector};

////////////////////////////////////////////////////////////////////////////////
// Point + Vector = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> Add<Vector<D>> for Point<D> {
    type Output = Point<D>;
    fn add(self, rhs: Vector<D>) -> Self::Output {
        Point(self.0.add(&rhs.0))
    }
}
impl<const D: usize> Add<&Vector<D>> for Point<D> {
    type Output = Point<D>;
    fn add(self, rhs: &Vector<D>) -> Self::Output {
        Point(self.0.add(&rhs.0))
    }
}
impl<const D: usize> Add<Vector<D>> for &Point<D> {
    type Output = Point<D>;
    fn add(self, rhs: Vector<D>) -> Self::Output {
        Point(self.0.add(&rhs.0))
    }
}
impl<const D: usize> Add<&Vector<D>> for &Point<D> {
    type Output = Point<D>;
    fn add(self, rhs: &Vector<D>) -> Self::Output {
        Point(self.0.add(&rhs.0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point += Vector /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> AddAssign<Vector<D>> for Point<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.0.add_assign(&rhs.0);
    }
}
impl<const D: usize> AddAssign<&Vector<D>> for Point<D> {
    fn add_assign(&mut self, rhs: &Vector<D>) {
        self.0.add_assign(&rhs.0);
    }
}
impl<const D: usize> AddAssign<Vector<D>> for &mut Point<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.0.add_assign(&rhs.0);
    }
}
impl<const D: usize> AddAssign<&Vector<D>> for &mut Point<D> {
    fn add_assign(&mut self, rhs: &Vector<D>) {
        self.0.add_assign(&rhs.0);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point - Point = Vector //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<const D: usize> Sub<Point<D>> for Point<D> {
    type Output = Vector<D>;
    fn sub(self, rhs: Point<D>) -> Self::Output {
        Vector(self.0.sub(&rhs.0))
    }
}
impl<const D: usize> Sub<&Point<D>> for Point<D> {
    type Output = Vector<D>;
    fn sub(self, rhs: &Point<D>) -> Self::Output {
        Vector(self.0.sub(&rhs.0))
    }
}
impl<const D: usize> Sub<Point<D>> for &Point<D> {
    type Output = Vector<D>;
    fn sub(self, rhs: Point<D>) -> Self::Output {
        Vector(self.0.sub(&rhs.0))
    }
}
impl<const D: usize> Sub<&Point<D>> for &Point<D> {
    type Output = Vector<D>;
    fn sub(self, rhs: &Point<D>) -> Self::Output {
        Vector(self.0.sub(&rhs.0))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point * Scaler = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T: Into<f64>, const D: usize> Mul<T> for Point<D> {
    type Output = Point<D>;
    fn mul(self, rhs: T) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}
impl<T: Into<f64>, const D: usize> Mul<T> for &Point<D> {
    type Output = Point<D>;
    fn mul(self, rhs: T) -> Self::Output {
        Point(self.0.mul(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point *= Scaler /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T: Into<f64>, const D: usize> MulAssign<T> for Point<D> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
    }
}
impl<T: Into<f64>, const D: usize> MulAssign<T> for &mut Point<D> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point / Scaler = Point //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T: Into<f64>, const D: usize> Div<T> for Point<D> {
    type Output = Point<D>;
    fn div(self, rhs: T) -> Self::Output {
        Point(self.0.div(rhs))
    }
}
impl<T: Into<f64>, const D: usize> Div<T> for &Point<D> {
    type Output = Point<D>;
    fn div(self, rhs: T) -> Self::Output {
        Point(self.0.div(rhs))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Point /= Scaler /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////
impl<T: Into<f64>, const D: usize> DivAssign<T> for Point<D> {
    fn div_assign(&mut self, rhs: T) {
        self.0.div_assign(rhs);
    }
}
impl<T: Into<f64>, const D: usize> DivAssign<T> for &mut Point<D> {
    fn div_assign(&mut self, rhs: T) {
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
