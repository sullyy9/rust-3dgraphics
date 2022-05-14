//! Implementation of a datatype that can be used to represent atomic 3D geometric elements such as points, vectors and
//! orientations.
//!

mod bounding_box;
mod dimension;
mod orientation;
mod orientation_vector;
mod point;
mod vector;

pub use self::{
    bounding_box::BBox, dimension::Dim, orientation::Orientation3D,
    orientation_vector::OrientationVector3D, point::Point, vector::Vector,
};
