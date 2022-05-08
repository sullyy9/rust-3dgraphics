//! Implementation of a 3D vector type.
//!

use crate::impl_coord_index;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use super::{atomic::Dim, point::Point};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a N dimensional vector.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector<const D: usize>(pub [f64; D]);

pub type Vector1D = Vector<1>;
pub type Vector2D = Vector<2>;
pub type Vector3D = Vector<3>;
pub type Vector4D = Vector<4>;

impl_coord_index! {impl Index for 1D type Vector1D}
impl_coord_index! {impl Index for 2D type Vector2D}
impl_coord_index! {impl Index for 3D type Vector3D}
impl_coord_index! {impl Index for 4D type Vector4D}

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Default for Vector<D> {
    fn default() -> Self {
        Self([0.0; D])
    }
}

impl<const D: usize> Vector<D> {
    /// Return a new Vector, given it's x, y and z components.
    ///
    pub fn new<T>(components: [T; D]) -> Self
    where
        T: Into<f64>,
    {
        Self(components.map(|comp| comp.into()))
    }

    /// Return a new Vector giving the magnitude and direction of one point to another.
    ///
    pub fn from_points(tail: Point<D>, head: Point<D>) -> Vector<D> {
        head.sub(tail)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Vector<D> {
    /// Return a new Vector3D object, normal to the 2 given vectors.
    ///
    pub fn normal_to(vector1: Vector<D>, vector2: Vector<D>) -> Vector<D> {
        // Calculate the cross product of the 2 given vectors to get a vector perpendicular to
        // both.
        let mut normal_vector: Vector<D> = Vector::default();
        normal_vector.0[0] = (vector1.0[1] * vector2.0[2]) - (vector1.0[2] * vector2.0[1]);
        normal_vector.0[1] = (vector1.0[2] * vector2.0[0]) - (vector1.0[0] * vector2.0[2]);
        normal_vector.0[2] = (vector1.0[0] * vector2.0[1]) - (vector1.0[1] * vector2.0[0]);

        // Normalise the vector (It's magnitude should be 1).
        normal_vector /= f64::sqrt(
            normal_vector.0[0].powi(2) + normal_vector.0[1].powi(2) + normal_vector.0[2].powi(2),
        );
        normal_vector
    }

    /// Return the magnitude of the vector.
    ///
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.iter().fold(0.0, |sum, coord| sum + coord.powi(2)))
    }

    /// Promote a vector to a higher dimentional vector where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const NEW: usize>(&self) -> Vector<NEW> {
        let mut new_vector = Vector::default();

        new_vector.0[..self.0.len()].clone_from_slice(&self.0);
        new_vector
    }

    /// Promote a vector to a lower dimentional vector.
    ///
    pub fn demote<const NEW: usize>(&self) -> Vector<NEW> {
        let mut new_vector = Vector::default();
        let len = new_vector.0.len();
        new_vector.0.clone_from_slice(&self.0[..len]);
        new_vector
    }

    /// Returns an iterator over the vector's coordinates.
    ///
    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    /// Returns an iterator over the vector's coordinates that allows modifying each value.
    ///
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }
}

/// Vector + Vector = Vector
///
impl<const D: usize> Add<&Vector<D>> for Vector<D> {
    type Output = Self;

    fn add(self, rhs: &Vector<D>) -> Self::Output {
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
impl<const D: usize> AddAssign for Vector<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(new_comp, rhs_comp)| *new_comp += rhs_comp);
    }
}

/// Scaler Arithmetic.
///
/// Vector * Scaler = Vector.
///
impl<T: Into<f64>, const D: usize> Mul<T> for Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.mul(rhs)))
    }
}
impl<T: Into<f64>, const D: usize> Mul<T> for &Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.mul(rhs)))
    }
}

/// Vector *= Scaler.
///
impl<T: Into<f64>, const D: usize> MulAssign<T> for Vector<D> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.mul_assign(rhs));
    }
}
impl<T: Into<f64>, const D: usize> MulAssign<T> for &mut Vector<D> {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.mul_assign(rhs));
    }
}

/// Vector / Scaler = Vector.
///
impl<T: Into<f64>, const D: usize> Div<T> for Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.div(rhs)))
    }
}
impl<T: Into<f64>, const D: usize> Div<T> for &Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::Output::new(self.0.map(|coord| coord.div(rhs)))
    }
}

/// Vector /= Scaler.
///
impl<T: Into<f64>, const D: usize> DivAssign<T> for Vector<D> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.div_assign(rhs));
    }
}
impl<T: Into<f64>, const D: usize> DivAssign<T> for &mut Vector<D> {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.iter_mut().for_each(|coord| coord.div_assign(rhs));
    }
}

/// -Vector = Vector
///
impl<const D: usize> Neg for Vector<D> {
    type Output = Vector<D>;

    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.map(|coord| coord.neg()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaler_mul() {
        let coords = [0.43, 56.28, -87.52, -0.23];
        let scaler = 4.87;

        let coords_scaled = coords.map(|coord| coord * 4.87);

        assert_eq!(Vector4D::new(coords) * scaler, Vector4D::new(coords_scaled));

        let mut point_mul_assign = Vector4D::new(coords);
        point_mul_assign *= scaler;
        assert_eq!(point_mul_assign, Vector4D::new(coords_scaled));
    }

    #[test]
    fn test_scaler_div() {
        let coords = [0.43, 56.28, -87.52, -0.23];
        let scaler = 4.87;

        let coords_scaled = coords.map(|coord| coord / 4.87);

        assert_eq!(Vector4D::new(coords) / scaler, Vector4D::new(coords_scaled));

        let mut point_mul_assign = Vector4D::new(coords);
        point_mul_assign /= scaler;
        assert_eq!(point_mul_assign, Vector4D::new(coords_scaled));
    }
}
