//! Implementation of a 3D point type.
//!

use super::{
    bounding_box::BoundingBox,
    vector::{Vector, Vector3D},
};
use crate::impl_scaler_arithmetic;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a 3D Point.
///
#[derive(Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Trait containg common behavior for all point types.
///
pub trait Point {
    type VectorType;

    fn get_x(&self) -> f64 {
        0.0
    }
    fn get_y(&self) -> f64 {
        0.0
    }
    fn get_z(&self) -> f64 {
        0.0
    }

    fn vector_from(&self, point: &impl Point) -> Self::VectorType;
    fn vector_to(&self, point: &impl Point) -> Self::VectorType;

    fn bound_by(&self, bbox: &BoundingBox) -> bool;
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for Point3D {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl Point3D {
    /// Return a new point3D object, given it's x, y and z components.
    ///
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Point3D
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Point3D {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl Point for Point3D {
    type VectorType = Vector3D;

    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_z(&self) -> f64 {
        self.z
    }

    /// Return a Vector3D describing the transformation from the given point to this point.
    ///
    fn vector_from(&self, point: &impl Point) -> Self::VectorType {
        Vector3D::new(
            self.x - point.get_x(),
            self.y - point.get_y(),
            self.z - point.get_z(),
        )
    }

    /// Return a Vector3D describing the transformation from this point to the given point.
    ///
    fn vector_to(&self, point: &impl Point) -> Self::VectorType {
        Vector3D::new(
            point.get_x() - self.x,
            point.get_y() - self.y,
            point.get_z() - self.z,
        )
    }

    /// Return true if the point is bound by the bounding box
    fn bound_by(&self, bbox: &BoundingBox) -> bool {
        if (bbox.xmin <= self.x)
            && (self.x <= bbox.xmax)
            && (bbox.ymin <= self.y)
            && (self.y <= bbox.ymax)
            && (bbox.zmin <= self.z)
            && (self.z <= bbox.zmax)
        {
            true
        } else {
            false
        }
    }
}

/// Operator overides
///
/// Adding a vector to a point can results in a new point.
/// AddAssigning a vector to a point results in the point being moved.
impl<T: Vector> Add<T> for Point3D {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other.get_x(),
            y: self.y + other.get_y(),
            z: self.z + other.get_z(),
        }
    }
}
impl<T: Vector> AddAssign<T> for Point3D {
    fn add_assign(&mut self, other: T) {
        self.x += other.get_x();
        self.y += other.get_y();
        self.z += other.get_z();
    }
}

impl_scaler_arithmetic! {impl Mul for Point3D}
impl_scaler_arithmetic! {impl Div for Point3D}
impl_scaler_arithmetic! {impl MulAssign for Point3D}
impl_scaler_arithmetic! {impl DivAssign for Point3D}
