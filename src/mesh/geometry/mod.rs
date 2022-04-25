//! Implementation of a datatype that can be used to represent atomic 3D geometric elements such as points, vectors and
//! orientations.
//!

mod bounding_box;
mod impl_arithmetic;
mod orientation;
mod orientation_vector;
mod point;
mod vector;

pub use self::{
    bounding_box::BoundingBox,
    orientation::Orientation3D,
    orientation_vector::OrientationVector3D,
    point::{Point, Point3D},
    vector::{Vector, Vector3D},
};
