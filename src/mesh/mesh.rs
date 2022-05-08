//! Implementations of a face-vertex mesh data structure and methods construction methods.
//!

use crate::physics::PhysicalState;

use super::{
    geometry::{
        BoundingBox,
        Dim::{W, X, Y, Z},
        Point3D, Vector3D,
    },
    {IndexPoly, Matrix4X4, RefPoly, Vertex},
};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// The mesh consists of a number of verticies and polygons.
/// Each polygon's points are indexes into the verticies vector.
/// Each polygon also contains an index into the normal vector to its normal.
#[derive(Clone)]
pub struct Mesh {
    verticies: Vec<Vertex>,
    normals: Vec<Vector3D>,
    polygons: Vec<IndexPoly>,
    visible_polygons: Vec<IndexPoly>,

    pub physics: PhysicalState,
}

pub struct PolyIterator<'a> {
    vertex_list: &'a [Vertex],
    normal_list: &'a [Vector3D],
    polygon_list: &'a [IndexPoly],
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for Mesh {
    fn default() -> Self {
        let verticies = Vec::new();
        let normals = Vec::new();
        let polygons = Vec::new();
        let visible_polygons = Vec::new();

        let physical_state = PhysicalState::new();

        Self {
            verticies,
            normals,
            polygons,
            visible_polygons,
            physics: physical_state,
        }
    }
}

impl Mesh {
    /// Load a cube into the mesh.
    ///
    pub fn load_cube(&mut self, edge_length: f64) {
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        self.verticies.push(Vertex::new([neg, neg, pos, 1.0]));
        self.verticies.push(Vertex::new([pos, neg, pos, 1.0]));
        self.verticies.push(Vertex::new([neg, neg, neg, 1.0]));
        self.verticies.push(Vertex::new([pos, neg, neg, 1.0]));
        self.verticies.push(Vertex::new([neg, pos, pos, 1.0]));
        self.verticies.push(Vertex::new([pos, pos, pos, 1.0]));
        self.verticies.push(Vertex::new([neg, pos, neg, 1.0]));
        self.verticies.push(Vertex::new([pos, pos, neg, 1.0]));

        self.polygons.push(IndexPoly::new(2, 6, 7, 0));
        self.polygons.push(IndexPoly::new(2, 7, 3, 1));
        self.polygons.push(IndexPoly::new(3, 7, 5, 2));
        self.polygons.push(IndexPoly::new(3, 5, 1, 3));
        self.polygons.push(IndexPoly::new(1, 5, 4, 4));
        self.polygons.push(IndexPoly::new(1, 4, 0, 5));
        self.polygons.push(IndexPoly::new(0, 4, 6, 6));
        self.polygons.push(IndexPoly::new(0, 6, 2, 7));
        self.polygons.push(IndexPoly::new(6, 4, 5, 8));
        self.polygons.push(IndexPoly::new(6, 5, 7, 9));
        self.polygons.push(IndexPoly::new(0, 2, 3, 10));
        self.polygons.push(IndexPoly::new(0, 3, 1, 11));

        for _ in self.polygons.iter() {
            self.normals.push(Vector3D::new([0, 0, 0]));
        }
    }
}

impl Mesh {
    /// Create a new mesh that has been run through the pipeline and contains only the polygons that should be drawn.
    ///
    pub fn run_pipeline(&self, project_mat: &Matrix4X4, window_size: [f64; 2]) -> Mesh {
        let mut processed_mesh = self.clone();
        processed_mesh.apply_transformations();
        processed_mesh.find_normals();
        processed_mesh.project_to_ndc(project_mat);
        processed_mesh.polygons_in_view();
        processed_mesh.project_to_screen(window_size[0], window_size[1]);

        processed_mesh
    }

    /// Apply position and rotation transformations.
    ///
    pub fn apply_transformations(&mut self) {
        // Find the rotation matrix
        let rotation_matrix = Matrix4X4::new_rotation(self.physics.orientation.vector());

        let position_vector = self.physics.position.vector_from(&Point3D::new([0, 0, 0]));

        // Apply rotation then position to each vertex.
        for vertex in self.verticies.iter_mut() {
            *vertex = *vertex * rotation_matrix;
            vertex.translate(&position_vector.promote());
        }
    }

    /// Find the normal unit vectors of each polygon in the mesh.
    ///
    pub fn find_normals(&mut self) {
        for indexpoly in self.polygons.iter() {
            self.normals[indexpoly.normal] = {
                let vector1 = self.verticies[indexpoly.verticies[1]]
                    .vector_from(&self.verticies[indexpoly.verticies[0]])
                    .demote();
                let vector2 = self.verticies[indexpoly.verticies[2]]
                    .vector_from(&self.verticies[indexpoly.verticies[0]])
                    .demote();

                Vector3D::normal_to(vector1, vector2)
            }
        }
    }

    /// Project the mesh from camera space to NDC space by applying a projection matrix to each vertex
    ///
    pub fn project_to_ndc(&mut self, projection_matrix: &Matrix4X4) {
        for vertex in self.verticies.iter_mut() {
            *vertex = *vertex * (*projection_matrix);
            *vertex /= vertex[W];
        }
    }

    /// Copy any polygons that are at least partially within ndc space, into the visible polygon list.
    ///
    pub fn polygons_in_view(&mut self) {
        let ndc_space = BoundingBox::new(Point3D::new([-1, -1, -1]), Point3D::new([1, 1, 1]));

        for indexpoly in self.polygons.iter() {
            let vert1_bound = self.verticies[indexpoly.verticies[0]].bound_by(&ndc_space);
            let vert2_bound = self.verticies[indexpoly.verticies[1]].bound_by(&ndc_space);
            let vert3_bound = self.verticies[indexpoly.verticies[2]].bound_by(&ndc_space);

            if vert1_bound || vert2_bound || vert3_bound {
                self.visible_polygons.push(indexpoly.to_owned());
            }
        }
    }

    /// Project the mesh from NDC space to screen space
    ///
    pub fn project_to_screen(&mut self, screen_width: f64, screen_height: f64) {
        let screen_width_mul = screen_width / 2.0;
        let screen_height_mul = screen_height / 2.0;
        let screen_depth_mul = 1000.0;

        for vertex in self.verticies.iter_mut() {
            vertex[X] = (vertex[X] + 1.0) * screen_width_mul;
            vertex[Y] = (vertex[Y] + 1.0) * screen_height_mul;
            vertex[Z] = screen_depth_mul - (vertex[Z] * screen_depth_mul);
        }
    }
}

#[allow(dead_code)]
impl Mesh {
    /// Iterate over all polygons immutably.
    ///
    pub fn iter_all_polygons(&self) -> PolyIterator {
        let vertex_list = self.verticies.as_slice();
        let normal_list = self.normals.as_slice();
        let polygon_list = self.polygons.as_slice();

        PolyIterator {
            vertex_list,
            normal_list,
            polygon_list,
        }
    }

    /// Iterate over only visible polygons immutably.
    ///
    pub fn iter_visible_polygons(&self) -> PolyIterator {
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
    type Item = RefPoly<'a>;

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
                RefPoly::new(
                    &self.vertex_list[index_poly.verticies[0]],
                    &self.vertex_list[index_poly.verticies[1]],
                    &self.vertex_list[index_poly.verticies[2]],
                    &self.normal_list[index_poly.normal],
                )
            };

            Some(ref_polygon)
        }
    }
}
