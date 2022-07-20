//! Implementation of a 3D orientation vector type.
//!

/// Wrapper type of Atomic3D used to represent 3D points.
///
#[derive(Copy, Clone)]
pub struct OrientationVector3D{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl OrientationVector3D {
    /// Return a new Vector3D object, given it's x, y and z components.
    ///
    pub fn new<T, U, V>(x: T, y: U, z: V) -> OrientationVector3D
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        OrientationVector3D{
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}