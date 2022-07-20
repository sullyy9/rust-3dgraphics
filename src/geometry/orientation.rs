//! Implementation of a 3D orientation type.
//!

use super::orientation_vector::OrientationVector3D;
use std::ops::{Add, AddAssign};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a 3D Orientation.
///
#[derive(Copy, Clone)]
pub struct Orientation3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for Orientation3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}
impl Orientation3D {
    /// Return a new Vector3D object, given it's x, y and z components.
    ///
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Orientation3D
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Orientation3D {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    /// Returns a vector from the origin to this point.
    pub fn vector(&self) -> OrientationVector3D {
        OrientationVector3D::new(self.x, self.y, self.z)
    }
}

/// Operator overides
///
impl Add<OrientationVector3D> for Orientation3D {
    type Output = Self;

    fn add(self, other: OrientationVector3D) -> Self {
        Self {
            x: (((self.x + other.x) % 360.0) + 360.0) % 360.0,
            y: (((self.y + other.y) % 360.0) + 360.0) % 360.0,
            z: (((self.z + other.z) % 360.0) + 360.0) % 360.0,
        }
    }
}
impl AddAssign<OrientationVector3D> for Orientation3D {
    fn add_assign(&mut self, other: OrientationVector3D) {
        self.x = (((self.x + other.x) % 360.0) + 360.0) % 360.0;
        self.y = (((self.y + other.y) % 360.0) + 360.0) % 360.0;
        self.z = (((self.z + other.z) % 360.0) + 360.0) % 360.0;
    }
}
