//! Module defining all geometric constructs and operations on/between them. 
//!

mod bounding_box;
mod dimension;
mod matrix;
mod orientation;
mod orientation_vector;
mod point;
mod scalar;
mod vector;

// External re-exports.
pub use self::{
    bounding_box::BBox, dimension::Dim, matrix::Matrix, orientation::Orientation3D,
    orientation_vector::OrientationVector3D, point::Point, vector::Vector, scalar::Scalar,
};
