//! Implementation of a face-vertex mesh.
//!

use super::Renderable;
// Re-imports.
//
pub(self) use super::{
    geometry::{Point, Vector},
    Bounding, Dim, Polygonal, Scalar, Transform,
};

mod iter;
mod pipeline;
mod polygon;

// Re-exports.
//
pub use self::{
    pipeline::Pipeline,
    polygon::{Polygon, Visibility},
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    marker::PhantomData,
    path::Path,
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

impl Mesh {
    pub fn new(path: &Path) -> Self {
        let mut vertex = Vec::default();
        let mut index = Vec::default();

        if let Ok(file) = File::open(path) {
            BufReader::new(file).lines().for_each(|line| {
                if let Ok(line) = line {
                    if line.starts_with('v') {
                        let coord: Vec<f64> = line
                            .trim()
                            .split_whitespace()
                            .skip(1)
                            .flat_map(str::parse::<f64>)
                            .collect();
                        vertex.push(Point::new([coord[0], coord[1], coord[2]]));
                    } else if line.starts_with('f') {
                        let indicies: Vec<usize> = line
                            .trim()
                            .split_whitespace()
                            .skip(1)
                            .flat_map(str::parse::<usize>)
                            .collect();
                        index.push(VIndex([indicies[0] - 1, indicies[1] - 1, indicies[2] - 1]));
                    }
                }
            });
        } else {
            panic!();
        }

        Mesh {
            vertex: vertex.into_boxed_slice(),
            vindex: index.into_boxed_slice(),
        }
    }

    /// Create a new Mesh which contains a cube.
    ///
    pub fn new_cube(edge_length: f64) -> Self {
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        let mut vertex = Vec::default();
        let mut index = Vec::default();

        vertex.push(Point::new([neg, neg, pos]));
        vertex.push(Point::new([pos, neg, pos]));
        vertex.push(Point::new([neg, neg, neg]));
        vertex.push(Point::new([pos, neg, neg]));
        vertex.push(Point::new([neg, pos, pos]));
        vertex.push(Point::new([pos, pos, pos]));
        vertex.push(Point::new([neg, pos, neg]));
        vertex.push(Point::new([pos, pos, neg]));

        index.push(VIndex([2, 6, 7]));
        index.push(VIndex([2, 7, 3]));
        index.push(VIndex([3, 7, 5]));
        index.push(VIndex([3, 5, 1]));
        index.push(VIndex([1, 5, 4]));
        index.push(VIndex([1, 4, 0]));
        index.push(VIndex([0, 4, 6]));
        index.push(VIndex([0, 6, 2]));
        index.push(VIndex([6, 4, 5]));
        index.push(VIndex([6, 5, 7]));
        index.push(VIndex([0, 2, 3]));
        index.push(VIndex([0, 3, 1]));

        Mesh {
            vertex: vertex.into_boxed_slice(),
            vindex: index.into_boxed_slice(),
        }
    }
}

impl Renderable<PipeMesh> for Mesh {
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
