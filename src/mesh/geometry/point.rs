//! Implementation of a 3D point type.
//!

use super::{
    atomic_traits::{Atomic, Atomic1D, Atomic2D, Atomic3D, Atomic4D},
    bounding_box::BoundingBox,
    vector::VectorBase,
    Vector,
};
use crate::{impl_atomic, impl_atomic_helper};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing N dimensional points.
///
#[derive(PartialEq, Debug, Clone)]
pub struct PointBase<const DIM: usize>(pub [f64; DIM]);

pub type Point3D = PointBase<3>;
pub type Point4D = PointBase<4>;

/// Trait containg common behavior for all point types.
///
pub trait Point<const DIM: usize> {
    fn vector_from(&self, point: &PointBase<DIM>) -> VectorBase<DIM>;
    fn vector_to(&self, point: &PointBase<DIM>) -> VectorBase<DIM>;
    fn translate(&mut self, vector: &VectorBase<DIM>);

    fn bound_by(&self, bbox: &BoundingBox) -> bool;

    fn iter(&self) -> std::slice::Iter<'_, f64>;
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64>;
}

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const DIM: usize> Default for PointBase<DIM> {
    fn default() -> Self {
        Self([0.0; DIM])
    }
}

impl<const DIM: usize> PointBase<DIM> {
    /// Return a new point3D object, given it's x, y and z components.
    ///
    pub fn new<T>(coords: [T; DIM]) -> Self
    where
        T: Into<f64>,
    {
        Self(coords.map(|coord| coord.into()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl_atomic! {impl Atomic3D for Point3D}
impl_atomic! {impl Atomic4D for Point4D}

impl<const DIM: usize> Point<DIM> for PointBase<DIM> {
    /// Return a Vector3D describing the transformation from the given point to this point.
    ///
    fn vector_from(&self, point_from: &PointBase<DIM>) -> VectorBase<DIM> {
        self.sub(point_from)
    }

    /// Return a Vector3D describing the transformation from this point to the given point.
    ///
    fn vector_to(&self, point_to: &PointBase<DIM>) -> VectorBase<DIM> {
        point_to.sub(self)
    }

    /// Translate a point by adding the given vector.
    ///
    fn translate(&mut self, vector: &VectorBase<DIM>) {
        self.add_assign(vector);
    }

    /// Return true if the point is bound by the bounding box
    fn bound_by(&self, bbox: &BoundingBox) -> bool {
        (bbox.xmin <= self.0[0])
            && (self.0[0] <= bbox.xmax)
            && (bbox.ymin <= self.0[1])
            && (self.0[1] <= bbox.ymax)
            && (bbox.zmin <= self.0[2])
            && (self.0[2] <= bbox.zmax)
    }

    fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Point + Vector = Point.
///
impl<const DIM: usize> Add<VectorBase<DIM>> for PointBase<DIM> {
    type Output = PointBase<DIM>;

    fn add(self, rhs: VectorBase<DIM>) -> Self::Output {
        let mut pt = self.clone();

        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}
impl<const DIM: usize> Add<&VectorBase<DIM>> for PointBase<DIM> {
    type Output = PointBase<DIM>;

    fn add(self, rhs: &VectorBase<DIM>) -> Self::Output {
        let mut pt = self;
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}
impl<const DIM: usize> Add<VectorBase<DIM>> for &PointBase<DIM> {
    type Output = PointBase<DIM>;

    fn add(self, rhs: VectorBase<DIM>) -> Self::Output {
        let mut pt = self.clone();
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}
impl<const DIM: usize> Add<&VectorBase<DIM>> for &PointBase<DIM> {
    type Output = PointBase<DIM>;

    fn add(self, rhs: &VectorBase<DIM>) -> Self::Output {
        let mut pt = self.clone();
        pt.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
        pt
    }
}

/// Point += Vector.
///
impl<const DIM: usize> AddAssign<VectorBase<DIM>> for PointBase<DIM> {
    fn add_assign(&mut self, rhs: VectorBase<DIM>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<const DIM: usize> AddAssign<&VectorBase<DIM>> for PointBase<DIM> {
    fn add_assign(&mut self, rhs: &VectorBase<DIM>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<const DIM: usize> AddAssign<VectorBase<DIM>> for &mut PointBase<DIM> {
    fn add_assign(&mut self, rhs: VectorBase<DIM>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}
impl<const DIM: usize> AddAssign<&VectorBase<DIM>> for &mut PointBase<DIM> {
    fn add_assign(&mut self, rhs: &VectorBase<DIM>) {
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.add_assign(rhs));
    }
}

/// Point - Point = Vector.
///
impl<const DIM: usize> Sub<PointBase<DIM>> for PointBase<DIM> {
    type Output = VectorBase<DIM>;

    fn sub(self, rhs: PointBase<DIM>) -> Self::Output {
        let mut vector = VectorBase(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}
impl<const DIM: usize> Sub<&PointBase<DIM>> for PointBase<DIM> {
    type Output = VectorBase<DIM>;

    fn sub(self, rhs: &PointBase<DIM>) -> Self::Output {
        let mut vector = VectorBase(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}
impl<const DIM: usize> Sub<PointBase<DIM>> for &PointBase<DIM> {
    type Output = VectorBase<DIM>;

    fn sub(self, rhs: PointBase<DIM>) -> Self::Output {
        let mut vector = VectorBase(self.0);
        vector
            .iter_mut()
            .zip(rhs.iter())
            .for_each(|(lhs, rhs)| lhs.sub_assign(rhs));
        vector
    }
}
impl<const DIM: usize> Sub<&PointBase<DIM>> for &PointBase<DIM> {
    type Output = VectorBase<DIM>;

    fn sub(self, rhs: &PointBase<DIM>) -> Self::Output {
        let mut vector = VectorBase(self.0);
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
impl<T: Into<f64>, const DIM: usize> Mul<T> for PointBase<DIM> {
    type Output = PointBase<DIM>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.mul(rhs)))
    }
}
impl<T: Into<f64>, const DIM: usize> Mul<T> for &PointBase<DIM> {
    type Output = PointBase<DIM>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.mul(rhs)))
    }
}

/// Point *= Scaler.
///
impl<T: Into<f64>, const DIM: usize> MulAssign<T> for PointBase<DIM> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.mul_assign(rhs));
    }
}
impl<T: Into<f64>, const DIM: usize> MulAssign<T> for &mut PointBase<DIM> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.mul_assign(rhs));
    }
}

/// Point / Scaler = Point.
///
impl<T: Into<f64>, const DIM: usize> Div<T> for PointBase<DIM> {
    type Output = PointBase<DIM>;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.div(rhs)))
    }
}
impl<T: Into<f64>, const DIM: usize> Div<T> for &PointBase<DIM> {
    type Output = PointBase<DIM>;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.div(rhs)))
    }
}

/// Point /= Scaler.
///
impl<T: Into<f64>, const DIM: usize> DivAssign<T> for PointBase<DIM> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.div_assign(rhs));
    }
}
impl<T: Into<f64>, const DIM: usize> DivAssign<T> for &mut PointBase<DIM> {
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
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_scaler_mul() {
        let rand_data: [(f64, [f64; 3]); 10] = rand::thread_rng().gen();
        for (scaler, coords) in rand_data {
            assert_eq!(
                Point3D::new(coords) * scaler,
                Point3D::new(coords.map(|x| x * scaler))
            );
        }
    }

    #[test]
    fn test_scaler_mul_assign() {
        let data: [(f64, [f64; 3]); 10] = rand::thread_rng().gen();

        for (scaler, coords) in data {
            let mut point_test = Point3D::new(coords);
            point_test *= scaler;

            assert_eq!(point_test, Point3D::new(coords.map(|x| x * scaler)));
        }
    }

    #[test]
    fn test_scaler_div() {
        let data: [(f64, [f64; 3]); 10] = rand::thread_rng().gen();

        for (scaler, coords) in data {
            assert_eq!(
                Point3D::new(coords) / scaler,
                Point3D::new(coords.map(|x| x / scaler))
            );
        }
    }

    #[test]
    fn test_scaler_div_assign() {
        let data: [(f64, [f64; 3]); 10] = rand::thread_rng().gen();

        for (scaler, coords) in data {
            let mut point_test = Point3D::new(coords);
            point_test /= scaler;

            assert_eq!(point_test, Point3D::new(coords.map(|x| x / scaler)));
        }
    }
}
