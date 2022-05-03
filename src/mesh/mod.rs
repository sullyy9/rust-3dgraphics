//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

mod matrix;
mod polygon;
mod vertex;
mod mesh;
// mod static_mesh;
// mod dynamic_mesh;

pub mod geometry;
pub use self::{
    matrix::Matrix4X4,
    polygon::{IndexPoly, Polygon, RefPoly},
    vertex::Vertex,
    mesh::Mesh,
    // static_mesh::StaticMesh,
    // dynamic_mesh::DynamicMesh,
};
