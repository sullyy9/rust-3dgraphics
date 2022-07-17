//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

mod face_vertex;
mod polygonal;
mod transform;

pub mod geometry;
pub use self::{
    face_vertex::{Mesh, PipeMesh, Pipeline, Polygon, Visibility},
    geometry::{BBox, Bounding, Dim, Point, Scalar, Vector},
    polygonal::Polygonal,
    transform::Transform,
};

pub trait Renderable<ScreenMeshBuilder>
where
    ScreenMeshBuilder: Pipeline,
{
    type ScreenMeshBuilder;

    fn start_pipeline(&self) -> Self::ScreenMeshBuilder;
}
