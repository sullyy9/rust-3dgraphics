//! Implementation of a 3D vector type.
//!

use crate::impl_scaler_arithmetic;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a 3D Vector.
///
#[derive(Copy, Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Trait containg common behavior for all vector types.
///
pub trait Vector {
    fn get_x(&self) -> f64 {
        0.0
    }
    fn get_y(&self) -> f64 {
        0.0
    }
    fn get_z(&self) -> f64 {
        0.0
    }

    fn normal_to(vector1: Vector3D, vector2: Vector3D) -> Vector3D;
    fn magnitude(&self) -> f64;
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for Vector3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl Vector3D {
    /// Return a new Vector3D object, given it's x, y and z components.
    ///
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Vector3D
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Vector3D {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl Vector for Vector3D {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_z(&self) -> f64 {
        self.z
    }

    /// Return a new Vector3D object, normal to the 2 given vectors.
    ///
    fn normal_to(vector1: Vector3D, vector2: Vector3D) -> Vector3D {
        // Calculate the cross product of the 2 given vectors to get a vector perpendicular to both.
        let mut normal_vector = Vector3D {
            x: ((vector1.y * vector2.z) - (vector1.z * vector2.y)),
            y: ((vector1.z * vector2.x) - (vector1.x * vector2.z)),
            z: ((vector1.x * vector2.y) - (vector1.y * vector2.x)),
        };

        // Normalise the vector (It's magnitude should be 1).
        normal_vector /= normal_vector.magnitude();
        normal_vector
    }

    /// Return the magnitude of the vector.
    ///
    fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
}

/// Operator overides
///
/// Adding a vector to a vector results in a new vector.
///
impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Vector3D) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl_scaler_arithmetic! {impl Mul for Vector3D}
impl_scaler_arithmetic! {impl Div for Vector3D}
impl_scaler_arithmetic! {impl MulAssign for Vector3D}
impl_scaler_arithmetic! {impl DivAssign for Vector3D}
