//! Implementations of arithmetic operations on vectors.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg};

use super::Vector;

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
impl<T: Into<f64>, const D: usize> Mul<T> for Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<T: Into<f64>, const D: usize> Mul<T> for &Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}

/// Vector *= Scaler.
///
impl<T: Into<f64>, const D: usize> MulAssign<T> for Vector<D> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
    }
}
impl<T: Into<f64>, const D: usize> MulAssign<T> for &mut Vector<D> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
    }
}

/// Vector / Scaler = Vector.
///
impl<T: Into<f64>, const D: usize> Div<T> for Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: T) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}
impl<T: Into<f64>, const D: usize> Div<T> for &Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: T) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}

/// Vector /= Scaler.
///
impl<T: Into<f64>, const D: usize> DivAssign<T> for Vector<D> {
    fn div_assign(&mut self, rhs: T) {
        self.0.div_assign(rhs);
    }
}
impl<T: Into<f64>, const D: usize> DivAssign<T> for &mut Vector<D> {
    fn div_assign(&mut self, rhs: T) {
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
