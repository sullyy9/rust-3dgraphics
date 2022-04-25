use super::{
    geometry::{BoundingBox, Point, Vector, Vector3D},
    matrix::Matrix4X4,
};

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a mesh vertex
///
#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Vertex {
    /// Return a new Vertex object, given it's x, y, z and w coordinates.
    ///
    pub fn new<T, U, V, W>(x: T, y: U, z: V, w: W) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
        W: Into<f64>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
}

impl Point for Vertex {
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

    fn vector_from(&self, point: &impl Point) -> Self::VectorType {
        Vector3D::new(
            self.x - point.get_x(),
            self.y - point.get_y(),
            self.z - point.get_z(),
        )
    }

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
/// Adding a vector to a point results in a new point.
/// AddAssigning a vector to a point results in the point being moved.
impl<T: Vector> Add<T> for Vertex {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other.get_x(),
            y: self.y + other.get_y(),
            z: self.z + other.get_z(),
            w: self.w,
        }
    }
}
impl<T: Vector> AddAssign<T> for Vertex {
    fn add_assign(&mut self, other: T) {
        self.x += other.get_x();
        self.y += other.get_y();
        self.z += other.get_z();
    }
}

impl Mul<Matrix4X4> for Vertex {
    type Output = Vertex;

    fn mul(self, matrix: Matrix4X4) -> Self::Output {
        Self {
            x: self.x * matrix.0[0][0]
                + self.y * matrix.0[1][0]
                + self.z * matrix.0[2][0]
                + self.w * matrix.0[3][0],
            y: self.x * matrix.0[0][1]
                + self.y * matrix.0[1][1]
                + self.z * matrix.0[2][1]
                + self.w * matrix.0[3][1],
            z: self.x * matrix.0[0][2]
                + self.y * matrix.0[1][2]
                + self.z * matrix.0[2][2]
                + self.w * matrix.0[3][2],
            w: self.x * matrix.0[0][3]
                + self.y * matrix.0[1][3]
                + self.z * matrix.0[2][3]
                + self.w * matrix.0[3][3],
        }
    }
}

/// Operator overloads for scalers.
///
impl<T: Into<f64>> Mul<T> for Vertex {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Vertex {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl<T: Into<f64>> MulAssign<T> for Vertex {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}
impl<T: Into<f64>> Div<T> for Vertex {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Vertex {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl<T: Into<f64>> DivAssign<T> for Vertex {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
