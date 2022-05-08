//! Implementation of a 3D point type.
//!

use crate::impl_coord_index;

use super::{atomic::Dim, bounding_box::BoundingBox, vector::Vector};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing N dimensional points.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point<const D: usize>(pub [f64; D]);

pub type Point1D = Point<1>;
pub type Point2D = Point<2>;
pub type Point3D = Point<3>;
pub type Point4D = Point<4>;

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Default for Point<D> {
    fn default() -> Self {
        Self([0.0; D])
    }
}

impl<const D: usize> Point<D> {
    /// Return a new point3D object, given it's x, y and z components.
    ///
    pub fn new<T>(coords: [T; D]) -> Self
    where
        T: Into<f64>,
    {
        Self(coords.map(|coord| coord.into()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
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

    /// Return true if the point is bound by the bounding box
    pub fn bound_by(&self, bbox: &BoundingBox) -> bool {
        (bbox.xmin <= self.0[0])
            && (self.0[0] <= bbox.xmax)
            && (bbox.ymin <= self.0[1])
            && (self.0[1] <= bbox.ymax)
            && (bbox.zmin <= self.0[2])
            && (self.0[2] <= bbox.zmax)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }
}

impl_coord_index! {impl Index for 1D type Point1D}
impl_coord_index! {impl Index for 2D type Point2D}
impl_coord_index! {impl Index for 3D type Point3D}
impl_coord_index! {impl Index for 4D type Point4D}

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Point + Vector = Point.
///
impl<const D: usize> Add<Vector<D>> for Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        let mut pt = self;
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}
impl<const D: usize> Add<&Vector<D>> for Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: &Vector<D>) -> Self::Output {
        let mut pt = self;
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}
impl<const D: usize> Add<Vector<D>> for &Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        let mut pt = *self;
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}
impl<const D: usize> Add<&Vector<D>> for &Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: &Vector<D>) -> Self::Output {
        let mut pt = *self;
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}

/// Point += Vector.
///
impl<const D: usize> AddAssign<Vector<D>> for Point<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<const D: usize> AddAssign<&Vector<D>> for Point<D> {
    fn add_assign(&mut self, rhs: &Vector<D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<const D: usize> AddAssign<Vector<D>> for &mut Point<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<const D: usize> AddAssign<&Vector<D>> for &mut Point<D> {
    fn add_assign(&mut self, rhs: &Vector<D>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}

/// Point - Point = Vector.
///
impl<const D: usize> Sub<Point<D>> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        let mut vector = Vector::new(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}
impl<const D: usize> Sub<&Point<D>> for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: &Point<D>) -> Self::Output {
        let mut vector = Vector::new(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}
impl<const D: usize> Sub<Point<D>> for &Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Point<D>) -> Self::Output {
        let mut vector = Vector::new(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}
impl<const D: usize> Sub<&Point<D>> for &Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: &Point<D>) -> Self::Output {
        let mut vector = Vector::new(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}

/// Scaler Arithmetic.
///
/// Point * Scaler = Point.
///
impl<T: Into<f64>, const D: usize> Mul<T> for Point<D> {
    type Output = Point<D>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.mul(rhs)))
    }
}
impl<T: Into<f64>, const D: usize> Mul<T> for &Point<D> {
    type Output = Point<D>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.mul(rhs)))
    }
}

/// Point *= Scaler.
///
impl<T: Into<f64>, const D: usize> MulAssign<T> for Point<D> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.mul_assign(rhs));
    }
}
impl<T: Into<f64>, const D: usize> MulAssign<T> for &mut Point<D> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.mul_assign(rhs));
    }
}

/// Point / Scaler = Point.
///
impl<T: Into<f64>, const D: usize> Div<T> for Point<D> {
    type Output = Point<D>;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.div(rhs)))
    }
}
impl<T: Into<f64>, const D: usize> Div<T> for &Point<D> {
    type Output = Point<D>;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.div(rhs)))
    }
}

/// Point /= Scaler.
///
impl<T: Into<f64>, const D: usize> DivAssign<T> for Point<D> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.div_assign(rhs));
    }
}
impl<T: Into<f64>, const D: usize> DivAssign<T> for &mut Point<D> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.div_assign(rhs));
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::mesh::geometry::Vector4D;

    use super::*;

    #[test]
    fn test_scaler_mul() {
        let control_point = Point4D::new([0.44, 50.28, -88.62, -0.24]);
        let mut test_point = Point4D::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(test_point * 2, control_point);

        test_point *= 2;
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_scaler_div() {
        let control_point = Point4D::new([0.22, 25.14, -44.31, -0.12]);
        let mut test_point = Point4D::new([0.44, 50.28, -88.62, -0.24]);

        assert_eq!(test_point / 2, control_point);

        test_point /= 2;
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_vector_addition() {
        let control_point = Point4D::new([0.44, 50.28, -88.62, -0.24]);
        let vector = Vector4D::new([0.22, 25.14, -44.31, -0.12]);
        let mut test_point = Point4D::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(test_point + vector, control_point);

        test_point.translate(&vector);
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_point_subtraction() {
        let point1 = Point4D::new([0.22, 25.14, -44.31, -0.12]);
        let point2 = Point4D::new([0.44, 50.28, -88.62, -0.24]);
        let vector = Vector4D::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(point1.vector_to(&point2), vector);
        assert_eq!(point1.vector_from(&point2), -vector);
    }
}
