//! Implementation of a datatype that can be used to represent atomic 3D geometric elements such as points, vectors and
//! orientations.
//!

mod dimension;
mod bounding_box;
mod impl_arithmetic;
mod orientation;
mod orientation_vector;
mod point;
mod vector;

pub use self::{
    dimension::Dim, bounding_box::BoundingBox, orientation::Orientation3D,
    orientation_vector::OrientationVector3D, point::Point, vector::Vector,
};
