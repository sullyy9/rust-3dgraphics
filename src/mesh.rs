//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

mod face_vertex;
mod transform;
mod polygonal;

pub mod geometry;
pub use self::{
    face_vertex::{Pipeline, Mesh, PipeMesh, Visibility, Polygon},
    geometry::{BBox, Bounding, Dim, Point, Scalar, Vector},
    transform::Transform,
    polygonal::{Polygonal}

};
