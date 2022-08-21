//! Module defining all geometric constructs and operations on/between them.
//!

mod angle;
mod bounding_box;
mod dimension;
mod matrix;
mod orientation;
mod point;
mod scalar;
mod vector;
mod line;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Re-exported Items //////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
pub use self::{
    angle::{Angle, Degrees, Radians},
    bounding_box::{BBox, Bounding},
    dimension::Dim,
    matrix::{Matrix, MatrixElement},
    orientation::{Orientation, RotationAxis},
    point::Point,
    scalar::Scalar,
    vector::Vector,
    line::LineSegment,
};