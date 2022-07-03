//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

mod fvmesh;
mod polygon;
mod transform;
mod vertex;
mod face_vertex;

pub mod geometry;
pub use self::{
    fvmesh::Mesh,
    // static_mesh::StaticMesh,
    // dynamic_mesh::DynamicMesh,
    polygon::{IndexPoly, Polygon, RefPoly},
    transform::Transform,
    vertex::Vertex,
};
