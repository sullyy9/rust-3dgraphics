//! Implementations of a face-vertex mesh data structure and methods construction methods.
//!

use crate::{physics, matrix, primitives::vertex};

use crate::primitives;

/// The mesh consists of a number of verticies and polygons.
/// Each polygon's points are indexes into the verticies vector.
/// Each polygon also contains an index into the normal vector to its normal.
#[derive(Clone)]
pub struct Mesh {
    verticies: Vec<vertex::Vertex>,
    normals: Vec<vertex::Vector>,
    polygons: Vec<primitives::IndexPoly>,
    visible_polygons: Vec<primitives::IndexPoly>,

    pub physical_state: physics::PhysicalState,
}
impl Mesh {
    ///
    /// Create an empty mesh.
    ///
    pub fn new() -> Mesh {
        let verticies = Vec::new();
        let normals = Vec::new();
        let polygons = Vec::new();
        let visible_polygons = Vec::new();

        let physical_state = physics::PhysicalState::new();

        Mesh {
            verticies,
            normals,
            polygons,
            visible_polygons,
            physical_state,
        }
    }

    ///
    /// Load a cube into the mesh.
    ///
    pub fn load_cube(&mut self, edge_length: f64) {
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        self.verticies.push(vertex::Vertex::new(neg, neg, pos, 1.0));
        self.verticies.push(vertex::Vertex::new(pos, neg, pos, 1.0));
        self.verticies.push(vertex::Vertex::new(neg, neg, neg, 1.0));
        self.verticies.push(vertex::Vertex::new(pos, neg, neg, 1.0));
        self.verticies.push(vertex::Vertex::new(neg, pos, pos, 1.0));
        self.verticies.push(vertex::Vertex::new(pos, pos, pos, 1.0));
        self.verticies.push(vertex::Vertex::new(neg, pos, neg, 1.0));
        self.verticies.push(vertex::Vertex::new(pos, pos, neg, 1.0));

        self.polygons.push(primitives::IndexPoly::new(2, 6, 7, 0));
        self.polygons.push(primitives::IndexPoly::new(2, 7, 3, 1));
        self.polygons.push(primitives::IndexPoly::new(3, 7, 5, 2));
        self.polygons.push(primitives::IndexPoly::new(3, 5, 1, 3));
        self.polygons.push(primitives::IndexPoly::new(1, 5, 4, 4));
        self.polygons.push(primitives::IndexPoly::new(1, 4, 0, 5));
        self.polygons.push(primitives::IndexPoly::new(0, 4, 6, 6));
        self.polygons.push(primitives::IndexPoly::new(0, 6, 2, 7));
        self.polygons.push(primitives::IndexPoly::new(6, 4, 5, 8));
        self.polygons.push(primitives::IndexPoly::new(6, 5, 7, 9));
        self.polygons.push(primitives::IndexPoly::new(0, 2, 3, 10));
        self.polygons.push(primitives::IndexPoly::new(0, 3, 1, 11));

        for _ in self.polygons.iter() {
            self.normals.push(vertex::Vector::new(0.0, 0.0, 0.0, 0.0));
        }
    }
}

impl physics::PhysicalObject for Mesh {
    fn set_absolute_position(&mut self, x: f64, y: f64, z: f64) {
        self.physical_state.set_absolute_position(x, y, z);
    }

    fn set_relative_position(&mut self, x: f64, y: f64, z: f64) {
        self.physical_state.set_relative_position(x, y, z);
    }

    fn set_absolute_orientation(&mut self, x: f64, y: f64, z: f64) {
        self.physical_state.set_absolute_orientation(x, y, z);
    }

    fn set_relative_orientation(&mut self, x: f64, y: f64, z: f64) {
        self.physical_state.set_relative_orientation(x, y, z);
    }
}

impl Mesh {
    ///
    /// Create a new mesh that has been run through the pipeline and contains only the polygons that should be drawn.
    ///
    pub fn run_pipeline(&self, project_mat: &matrix::TransformMatrix, window_size: [f64; 2]) -> Mesh {
        let mut processed_mesh = self.clone();
        processed_mesh.apply_transformations();
        processed_mesh.find_normals();
        processed_mesh.project_to_ndc(project_mat);
        processed_mesh.polygons_in_view();
        processed_mesh.project_to_screen(window_size[0], window_size[1]);

        processed_mesh
    }

    ///
    /// Apply position and rotation transformations.
    ///
    pub fn apply_transformations(&mut self) {
        // Find the rotation matrix
        let rotation_matrix = {
            let pi = std::f64::consts::PI;
            let x_rot = self.physical_state.orientation.x * (pi / 180.0);
            let y_rot = self.physical_state.orientation.y * (pi / 180.0);
            let z_rot = self.physical_state.orientation.z * (pi / 180.0);

            matrix::TransformMatrix::new_rotation(x_rot, y_rot, z_rot)
        };

        let position = vertex::Vertex{
            x: self.physical_state.position.x,
            y: self.physical_state.position.y,
            z: self.physical_state.position.z,
            w: 0.0,
        };

        // Apply rotation then position to each vertex.
        for vertex in self.verticies.iter_mut() {
            *vertex = *vertex * rotation_matrix;
            *vertex = *vertex + position;
        }
    }

    ///
    /// Find the normal unit vectors of each polygon in the mesh.
    ///
    pub fn find_normals(&mut self) {
        for indexpoly in self.polygons.iter() {
            self.normals[indexpoly.normal] = {
                let vect1 = self.verticies[indexpoly.p2] - self.verticies[indexpoly.p1];
                let vect2 = self.verticies[indexpoly.p3] - self.verticies[indexpoly.p1];

                let mut norm_x = vect1.y * vect2.z - vect1.z * vect2.y;
                let mut norm_y = vect1.z * vect2.x - vect1.x * vect2.z;
                let mut norm_z = vect1.x * vect2.y - vect1.y * vect2.x;

                let divisor = f64::sqrt(norm_x * norm_x + norm_y * norm_y + norm_z * norm_z);
                norm_x /= divisor;
                norm_y /= divisor;
                norm_z /= divisor;

                vertex::Vector::new(norm_x, norm_y, norm_z, 0.0)
            }
        }
    }

    ///
    /// Project the mesh from camera space to NDC space by applying a projection matrix to each vertex
    ///
    pub fn project_to_ndc(&mut self, projection_matrix: &matrix::TransformMatrix) {
        for vertex in self.verticies.iter_mut() {
            //*vertex = *vertex + self.position;
            *vertex = *vertex * (*projection_matrix);
            *vertex = *vertex / vertex.w;
        }
    }

    ///
    /// Copy any polygons that are at least partially within ndc space, into the visible polygon list.
    ///
    pub fn polygons_in_view(&mut self) {
        for indexpoly in self.polygons.iter() {
            let vert1 = &self.verticies[indexpoly.p1];
            let vert2 = &self.verticies[indexpoly.p2];
            let vert3 = &self.verticies[indexpoly.p3];

            if vert1.in_ndc_space() || vert2.in_ndc_space() || vert3.in_ndc_space() {
                self.visible_polygons.push(indexpoly.to_owned());
            }
        }
    }

    ///
    /// Project the mesh from NDC space to screen space
    ///
    pub fn project_to_screen(&mut self, screen_width: f64, screen_height: f64) {
        let screen_width_mul = screen_width as f64 / 2.0;
        let screen_height_mul = screen_height as f64 / 2.0;
        let screen_depth_mul = 1000.0;

        for vertex in self.verticies.iter_mut() {
            vertex.x = (vertex.x + 1.0) * screen_width_mul;
            vertex.y = (vertex.y + 1.0) * screen_height_mul;
            vertex.z = screen_depth_mul - (vertex.z * screen_depth_mul);
        }
    }
}

pub struct PolyIterator<'a> {
    vertex_list: &'a [vertex::Vertex],
    normal_list: &'a [vertex::Vector],
    polygon_list: &'a [primitives::IndexPoly],
}
#[allow(dead_code)]
impl Mesh {
    ///
    /// Iterate over all polygons immutably.
    ///
    pub fn iter_all_polygons<'a>(&'a self) -> PolyIterator<'a> {
        let vertex_list = self.verticies.as_slice();
        let normal_list = self.normals.as_slice();
        let polygon_list = self.polygons.as_slice();

        PolyIterator {
            vertex_list,
            normal_list,
            polygon_list,
        }
    }

    ///
    /// Iterate over only visible polygons immutably.
    ///
    pub fn iter_visible_polygons<'a>(&'a self) -> PolyIterator<'a> {
        let vertex_list = self.verticies.as_slice();
        let normal_list = self.normals.as_slice();
        let polygon_list = self.visible_polygons.as_slice();

        PolyIterator {
            vertex_list,
            normal_list,
            polygon_list,
        }
    }
}
impl<'a> Iterator for PolyIterator<'a> {
    type Item = primitives::RefPoly<'a>;

    ///
    /// Get the next item.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.polygon_list.is_empty() {
            None
        } else {
            let ref_polygon = {
                // Split off a reference to the first polygon in the slice and give the reference to the rest of the
                // list back to the iterator struct.
                let (index_poly, remaining_list) = self.polygon_list.split_first()?;
                self.polygon_list = remaining_list;

                // Construct a polygon of references from the index polygon and vertex list.
                primitives::RefPoly::new(
                    &self.vertex_list[index_poly.p1],
                    &self.vertex_list[index_poly.p2],
                    &self.vertex_list[index_poly.p3],
                    &self.normal_list[index_poly.normal],
                )
            };

            Some(ref_polygon)
        }
    }
}