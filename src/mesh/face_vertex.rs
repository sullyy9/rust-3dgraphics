//! Implementation of a face-vertex mesh.
//!

mod construct;
mod iter;
mod pipeline;
mod polygon;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Imported Items /////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

pub(self) use super::{
    geometry::{Bounding, Dim, Point, Scalar, Vector},
    {Polygonal, Transform},
};

use super::Renderable;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Re-exported Items //////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

pub use self::{
    pipeline::Pipeline,
    polygon::{Polygon, Visibility},
};

#[derive(Clone)]
struct VIndex(pub(self) [usize; 3]);

/// Face-vertex mesh type.
pub struct Mesh {
    vertex: Box<[Point<3>]>,
    vindex: Box<[VIndex]>,
}

pub struct PipeMesh {
    vertex: Box<[Point<4>]>,
    vindex: Box<[VIndex]>,
    normal: Option<Box<[Vector<3>]>>,
    visible: Option<Box<[Visibility]>>,
}

impl Renderable for Mesh {
    type ScreenMeshBuilder = PipeMesh;

    fn start_pipeline(&self) -> Self::ScreenMeshBuilder {
        let vertex = self
            .vertex
            .iter()
            .map(|vertex| Point::new([vertex[Dim::X], vertex[Dim::Y], vertex[Dim::Z], 1.0]))
            .collect();
        PipeMesh {
            vertex,
            vindex: self.vindex.clone(),
            normal: None,
            visible: None,
        }
    }
}

impl PipeMesh {
    pub fn iter(&self) -> iter::Iter {
        iter::Iter::new(self)
    }
}
