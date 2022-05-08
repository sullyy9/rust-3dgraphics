//! Implementation of a datatype that can be used to represent atomic 3D geometric elements such as points, vectors and
//! orientations.
//!

mod atomic;
mod bounding_box;
mod impl_arithmetic;
mod orientation;
mod orientation_vector;
mod point;
mod vector;

pub use self::{
    atomic::Dim,
    bounding_box::BoundingBox,
    orientation::Orientation3D,
    orientation_vector::OrientationVector3D,
    point::{Point3D, Point4D},
    vector::{Vector3D, Vector4D},
};
