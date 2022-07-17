//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

pub mod face_vertex;
mod transform;

pub mod geometry;
pub use self::{
    face_vertex::Pipeline,
    geometry::{BBox, Bounding, Dim, Point, Scalar, Vector},
    transform::Transform,
};
