//! Implementation of a 3D vector type.
//!

use crate::{impl_atomic, impl_atomic_helper};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use super::atomic_traits::{Atomic, Atomic1D, Atomic2D, Atomic3D, Atomic4D};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a N dimensional vector.
///
#[derive(Clone)]
pub struct VectorBase<const DIM: usize>(pub [f64; DIM]);

pub type Vector3D = VectorBase<3>;
pub type Vector4D = VectorBase<4>;

/// Trait containg common behavior for all vector types.
///
pub trait Vector<const DIM: usize> {
    // type PointType;

    fn normal_to(
        vector1: VectorBase<DIM>,
        vector2: VectorBase<DIM>,
    ) -> VectorBase<DIM>;

    fn magnitude(&self) -> f64;

    fn promote<const NEWDIM: usize>(&self) -> VectorBase<NEWDIM>;
    fn demote<const NEWDIM: usize>(&self) -> VectorBase<NEWDIM>;

    fn iter(&self) -> std::slice::Iter<'_, f64>;

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64>;
}

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const DIM: usize> Default for VectorBase<DIM> {
    fn default() -> Self {
        Self([0.0; DIM])
    }
}

impl<const DIM: usize> VectorBase<DIM> {
    /// Return a new point3D object, given it's x, y and z components.
    ///
    pub fn new<T>(components: [T; DIM]) -> Self
    where
        T: Into<f64>,
    {
        Self(components.map(|comp| comp.into()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl_atomic! {impl Atomic3D for Vector3D}
impl_atomic! {impl Atomic4D for Vector4D}

impl<const DIM: usize> Vector<DIM> for VectorBase<DIM> {
    // type PointType = Point3D;

    /// Return a new Vector3D object, normal to the 2 given vectors.
    ///
    fn normal_to(
        vector1: VectorBase<DIM>,
        vector2: VectorBase<DIM>,
    ) -> VectorBase<DIM> {
        // Calculate the cross product of the 2 given vectors to get a vector perpendicular to both.
        let mut normal_vector: VectorBase<DIM> = VectorBase::default();
        normal_vector.0[0] = (vector1.0[1] * vector2.0[2]) - (vector1.0[2] * vector2.0[1]);
        normal_vector.0[1] = (vector1.0[2] * vector2.0[0]) - (vector1.0[0] * vector2.0[2]);
        normal_vector.0[2] = (vector1.0[0] * vector2.0[1]) - (vector1.0[1] * vector2.0[0]);

        // Normalise the vector (It's magnitude should be 1).
        //normal_vector /= normal_vector.magnitude();
        normal_vector /= f64::sqrt(
            normal_vector.0[0].powi(2) + normal_vector.0[1].powi(2) + normal_vector.0[2].powi(2),
        );
        normal_vector
    }

    /// Return the magnitude of the vector.
    ///
    fn magnitude(&self) -> f64 {
        f64::sqrt(self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2))
    }

    /// Promote a vector to a higher dimentional vector where the additional dimensions are initialised as 0.
    ///
    fn promote<const NEWDIM: usize>(&self) -> VectorBase<NEWDIM> {
        let mut new_vector = VectorBase::default();

        new_vector.0[..self.0.len()].clone_from_slice(&self.0);
        new_vector
    }

    /// Promote a vector to a lower dimentional vector.
    ///
    fn demote<const NEWDIM: usize>(&self) -> VectorBase<NEWDIM> {
        let mut new_vector = VectorBase::default();
        let len = new_vector.0.len();
        new_vector.0.clone_from_slice(&self.0[..len]);
        new_vector
    }

    fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }
}

/// Vector + Vector = Vector
///
impl<const DIM: usize> Add<&VectorBase<DIM>> for VectorBase<DIM> {
    type Output = Self;

    fn add(self, rhs: &VectorBase<DIM>) -> Self::Output {
        let mut point = self;
        point
            .0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(new_comp, rhs_comp)| *new_comp += rhs_comp);
        point
    }
}

/// Vector += Vector
///
impl<const DIM: usize> AddAssign for VectorBase<DIM> {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(new_comp, rhs_comp)| *new_comp += rhs_comp);
    }
}

/// Scalar arithmetic.
///
impl<T: Into<f64>, const DIM: usize> Mul<T> for VectorBase<DIM> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Self(self.0.map(|comp| comp * rhs))
    }
}

impl<T: Into<f64>, const DIM: usize> MulAssign<T> for VectorBase<DIM> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0.iter_mut().for_each(|comp| *comp *= rhs);
    }
}

impl<T: Into<f64>, const DIM: usize> Div<T> for VectorBase<DIM> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Self(self.0.map(|comp| comp / rhs))
    }
}

impl<T: Into<f64>, const DIM: usize> DivAssign<T> for VectorBase<DIM> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0.iter_mut().for_each(|comp| *comp /= rhs);
    }
}
