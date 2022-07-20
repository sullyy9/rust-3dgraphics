use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::{Mesh, Point, VIndex};

impl Default for Mesh {
    /// Construct a new face-vertex mesh containing a cube with an edge length of 100.
    ///
    fn default() -> Self {
        let pos = 50.0;
        let neg = -50.0;

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

impl Mesh {
    /// Construct a new face-vertex mesh, reading the data from a .obj file.
    ///
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
}
