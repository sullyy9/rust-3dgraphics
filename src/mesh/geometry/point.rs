//! Implementation of Point types.
//!

use super::{dimension::Dim, matrix::Matrix, vector::Vector};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub};

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing N dimensional points.
///
#[derive(PartialEq, Debug, Clone, Copy)]
//pub struct Point<const D: usize>(pub [f64; D]);

pub struct Point<const D: usize>(Matrix<1, D>);

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Default for Point<D> {
    fn default() -> Self {
        Self(Matrix::new([[0.0; D]]))
    }
}

impl<const D: usize> Point<D> {
    /// Return a new point3D object, given it's x, y and z components.
    ///
    pub fn new<T>(coords: [T; D]) -> Self
    where
        T: Into<f64>,
    {
        Self(Matrix::new([coords.map(|coord| coord.into())]))
    }

    /// Promote a point to a higher dimentional point where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Point<ND> {
        let mut new_point = Point::default();
        let len = self.0[0].len();
        new_point.0[0][..len].clone_from_slice(&self.0[0]);
        new_point
    }

    /// Demote a point to a lower dimentional point.
    ///
    pub fn demote<const ND: usize>(&self) -> Point<ND> {
        let mut new_point = Point::default();
        let len = new_point.0[0].len();
        new_point.0[0].clone_from_slice(&self.0[0][..len]);
        new_point
    }
}

////////////////////////////////////////////////////////////////////////////////
// Method Implementations //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Point<D> {
    /// Return a Vector3D describing the transformation from the given point to this point.
    ///
    pub fn vector_from(&self, point_from: &Point<D>) -> Vector<D> {
        self.sub(point_from)
    }

    /// Return a Vector3D describing the transformation from this point to the given point.
    ///
    pub fn vector_to(&self, point_to: &Point<D>) -> Vector<D> {
        point_to.sub(self)
    }

    /// Translate a point by adding the given vector.
    ///
    pub fn translate(&mut self, vector: &Vector<D>) {
        self.add_assign(vector);
    }

    /// Return an iterator over a mutable slice, containing a point's coordinates.
    ///
    pub fn iter(&mut self) -> std::iter::Flatten<std::slice::Iter<'_, [f64; D]>> {
        self.0.iter()
    }

    /// Return an iterator over a mutable slice, containing a point's coordinates.
    ///
    pub fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [f64; D]>> {
        self.0.iter_mut()
    }

    /// Return a new point where each coordinate has been modified acording to the closure f.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    #[allow(dead_code)]
    fn map<F>(&self, f: F) -> Point<D>
    where
        F: Fn(f64) -> f64,
    {
        Point(self.0.map(f))
    }

    /// Apply the closure f to each of a point's coordinates.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    #[allow(dead_code)]
    fn for_each_coord<F>(&mut self, f: F)
    where
        F: Fn(&mut f64),
    {
        self.0.for_each(f);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<'a, const D: usize> IntoIterator for &'a Point<D> {
    type Item = f64;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[f64; D], 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const D: usize> AsRef<Matrix<1, D>> for Point<D> {
    fn as_ref(&self) -> &Matrix<1, D> {
        &self.0
    }
}

impl<const D: usize> From<Matrix<1, D>> for Point<D> {
    fn from(matrix: Matrix<1, D>) -> Self {
        Point(matrix)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Index<Dim> for Point<D> {
    type Output = f64;

    fn index(&self, index: Dim) -> &Self::Output {
        match index {
            Dim::X if D >= 1 => &self.0[0][0],
            Dim::Y if D >= 2 => &self.0[0][1],
            Dim::Z if D >= 3 => &self.0[0][2],
            Dim::W if D >= 4 => &self.0[0][3],
            Dim::N(n) if D >= n => &self.0[0][n],
            _ => panic!(),
        }
    }
}
impl<const D: usize> IndexMut<Dim> for Point<D> {
    fn index_mut(&mut self, index: Dim) -> &mut Self::Output {
        match index {
            Dim::X if D >= 1 => &mut self.0[0][0],
            Dim::Y if D >= 2 => &mut self.0[0][1],
            Dim::Z if D >= 3 => &mut self.0[0][2],
            Dim::W if D >= 4 => &mut self.0[0][3],
            Dim::N(n) if D >= n => &mut self.0[0][n],
            _ => panic!(),
        }
    }
}

/// Point + Vector = Point.
///
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

/// Point += Vector.
///
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

/// Point - Point = Vector.
///
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

/// Scaler Arithmetic.
///
/// Point * Scaler = Point.
///
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

/// Point *= Scaler.
///
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

/// Point / Scaler = Point.
///
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

/// Point /= Scaler.
///
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
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::mesh::geometry::Vector;

    use super::*;

    #[test]
    fn test_scaler_mul() {
        let control_point = Point::new([0.44, 50.28, -88.62, -0.24]);
        let mut test_point = Point::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(test_point * 2, control_point);

        test_point *= 2;
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_scaler_div() {
        let control_point = Point::new([0.22, 25.14, -44.31, -0.12]);
        let mut test_point = Point::new([0.44, 50.28, -88.62, -0.24]);

        assert_eq!(test_point / 2, control_point);

        test_point /= 2;
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_vector_addition() {
        let control_point = Point::new([0.44, 50.28, -88.62, -0.24]);
        let vector = Vector::new([0.22, 25.14, -44.31, -0.12]);
        let mut test_point = Point::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(test_point + vector, control_point);

        test_point.translate(&vector);
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_point_subtraction() {
        let point1 = Point::new([0.22, 25.14, -44.31, -0.12]);
        let point2 = Point::new([0.44, 50.28, -88.62, -0.24]);
        let vector = Vector::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(point1.vector_to(&point2), vector);
        assert_eq!(point1.vector_from(&point2), -vector);
    }
}
