//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

mod transform;
mod polygon;
mod vertex;
mod fvmesh;
// mod static_mesh;
// mod dynamic_mesh;

pub mod geometry;
pub use self::{
    transform::Matrix4X4,
    polygon::{IndexPoly, Polygon, RefPoly},
    vertex::Vertex,
    fvmesh::Mesh,
    // static_mesh::StaticMesh,
    // dynamic_mesh::DynamicMesh,
};
