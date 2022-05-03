//! Implementation of a 3D point type.
//!

use super::{
    atomic_traits::{Atomic, Atomic1D, Atomic2D, Atomic3D, Atomic4D},
    bounding_box::BoundingBox,
    vector::VectorBase,
};
use crate::{impl_atomic, impl_atomic_helper};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

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

    fn bound_by(&self, bbox: &BoundingBox) -> bool;
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
        self - point_from
    }

    /// Return a Vector3D describing the transformation from this point to the given point.
    ///
    fn vector_to(&self, point_to: &PointBase<DIM>) -> VectorBase<DIM> {
        point_to - self
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
}

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Point + Vector = Point.
///
impl<const DIM: usize> Add<&VectorBase<DIM>> for PointBase<DIM> {
    type Output = PointBase<DIM>;

    fn add(self, rhs: &VectorBase<DIM>) -> Self::Output {
        let mut point = self;
        point
            .0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(new_coord, rhs_coord)| *new_coord += rhs_coord);
        point
    }
}

/// Point += Vector.
///
impl<const DIM: usize> AddAssign<&VectorBase<DIM>> for PointBase<DIM> {
    fn add_assign(&mut self, rhs: &VectorBase<DIM>) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(new_coord, rhs_coord)| *new_coord += rhs_coord);
    }
}

/// Point - Point = Vector.
///
impl<const DIM: usize> Sub<&PointBase<DIM>> for &PointBase<DIM> {
    type Output = VectorBase<DIM>;

    fn sub(self, rhs: &PointBase<DIM>) -> Self::Output {
        let mut vector = VectorBase(self.0);
        vector
            .0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(new_coord, rhs_coord)| *new_coord -= rhs_coord);
        vector
    }
}

/// Scalar arithmetic.
///
impl<T: Into<f64>, const DIM: usize> Mul<T> for PointBase<DIM> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Self(self.0.map(|coord| coord * rhs))
    }
}

impl<T: Into<f64>, const DIM: usize> MulAssign<T> for PointBase<DIM> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0.iter_mut().for_each(|coord| *coord *= rhs);
    }
}

impl<T: Into<f64>, const DIM: usize> Div<T> for PointBase<DIM> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Self(self.0.map(|coord| coord / rhs))
    }
}

impl<T: Into<f64>, const DIM: usize> DivAssign<T> for PointBase<DIM> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0.iter_mut().for_each(|coord| *coord /= rhs);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Point3D;
    use rand::prelude::*;

    #[test]
    fn test_scaler_mul() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let scaler = rng.gen_range(-10000.0..10000.0);
            let coords = [rng.gen_range(-1000.0..1000.0); 3];

            let point_test = Point3D::new(coords) * scaler;
            let point_control = Point3D::new(coords.map(|x| x * scaler));

            assert_eq!(point_test, point_control);
        }
    }

    #[test]
    fn test_scaler_mul_assign() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let scaler = rng.gen_range(-10000.0..10000.0);
            let coords = [rng.gen_range(-1000.0..1000.0); 3];

            let mut point_test = Point3D::new(coords);
            point_test *= scaler;
            let point_control = Point3D::new(coords.map(|x| x * scaler));

            assert_eq!(point_test, point_control);
        }
    }

    #[test]
    fn test_scaler_div() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let scaler = rng.gen_range(-10000.0..10000.0);
            let coords = [rng.gen_range(-1000.0..1000.0); 3];

            let point_test = Point3D::new(coords) / scaler;
            let point_control = Point3D::new(coords.map(|x| x / scaler));

            assert_eq!(point_test, point_control);
        }
    }

    #[test]
    fn test_scaler_div_assign() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let scaler = rng.gen_range(-10000.0..10000.0);
            let coords = [rng.gen_range(-1000.0..1000.0); 3];

            let mut point_test = Point3D::new(coords);
            point_test /= scaler;
            let point_control = Point3D::new(coords.map(|x| x / scaler));

            assert_eq!(point_test, point_control);
        }
    }
}
