use crate::primitives as prim;

/// The mesh consists of a number of verticies and polygons. Each polygon contains 3 indicies
/// to the positions of its verticies in the verticies vector.
#[derive(Clone)]
pub struct Mesh {
    verticies: Vec<prim::Vertex>,
    normals: Vec<prim::Vector>,
    polygons: Vec<prim::IndexPoly>,

    visible_polygons: Vec<prim::IndexPoly>,

    pub position: prim::Vertex,
    pub orientation: prim::Vertex,
}
// Construction functions
impl Mesh {
    ///
    /// Create an empty mesh
    ///
    pub fn new() -> Mesh {
        let verticies = Vec::new();
        let normals = Vec::new();
        let polygons = Vec::new();
        let visible_polygons = Vec::new();

        let position = prim::Vertex::new(0.0, 0.0, 0.0, 0.0);
        let orientation = prim::Vertex::new(0.0, 0.0, 0.0, 0.0);

        Mesh {
            verticies,
            normals,
            polygons,
            visible_polygons,
            position,
            orientation,
        }
    }

    ///
    /// Load a cube into the mesh
    ///
    pub fn load_cube(&mut self, edge_length: f32) {
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        self.verticies.push(prim::Vertex::new(neg, neg, pos, 1.0));
        self.verticies.push(prim::Vertex::new(pos, neg, pos, 1.0));
        self.verticies.push(prim::Vertex::new(neg, neg, neg, 1.0));
        self.verticies.push(prim::Vertex::new(pos, neg, neg, 1.0));
        self.verticies.push(prim::Vertex::new(neg, pos, pos, 1.0));
        self.verticies.push(prim::Vertex::new(pos, pos, pos, 1.0));
        self.verticies.push(prim::Vertex::new(neg, pos, neg, 1.0));
        self.verticies.push(prim::Vertex::new(pos, pos, neg, 1.0));

        self.polygons.push(prim::IndexPoly::new(2, 6, 7, 0));
        self.polygons.push(prim::IndexPoly::new(2, 7, 3, 1));
        self.polygons.push(prim::IndexPoly::new(3, 7, 5, 2));
        self.polygons.push(prim::IndexPoly::new(3, 5, 1, 3));
        self.polygons.push(prim::IndexPoly::new(1, 5, 4, 4));
        self.polygons.push(prim::IndexPoly::new(1, 4, 0, 5));
        self.polygons.push(prim::IndexPoly::new(0, 4, 6, 6));
        self.polygons.push(prim::IndexPoly::new(0, 6, 2, 7));
        self.polygons.push(prim::IndexPoly::new(6, 4, 5, 8));
        self.polygons.push(prim::IndexPoly::new(6, 5, 7, 9));
        self.polygons.push(prim::IndexPoly::new(0, 2, 3, 10));
        self.polygons.push(prim::IndexPoly::new(0, 3, 1, 11));

        for _ in self.polygons.iter() {
            self.normals.push(prim::Vector::new(0.0, 0.0, 0.0, 0.0));
        }
    }
}

//
// Functions to translate or rotate the mesh
//
#[allow(dead_code)]
impl Mesh {
    ///
    /// Set the mesh's absolute orientation.
    ///
    pub fn abs_orientation(&mut self, x: f32, y: f32, z: f32) {
        self.orientation.x = x.clamp(-180.0, 180.0);
        self.orientation.y = y.clamp(-180.0, 180.0);
        self.orientation.z = z.clamp(-180.0, 180.0);
    }

    ///
    /// Set the mesh's absolute orientation.
    ///
    pub fn rel_orientation(&mut self, x: f32, y: f32, z: f32) {
        self.orientation.x += x;
        self.orientation.y += y;
        self.orientation.z += z;

        while self.orientation.x > 180.0 {
            self.orientation.x -= 360.0;
        }
        while self.orientation.x < 180.0 {
            self.orientation.x += 360.0;
        }

        while self.orientation.y > 180.0 {
            self.orientation.y -= 360.0;
        }
        while self.orientation.y < 180.0 {
            self.orientation.y += 360.0;
        }

        while self.orientation.y > 180.0 {
            self.orientation.y -= 360.0;
        }
        while self.orientation.y < 180.0 {
            self.orientation.y += 360.0;
        }
    }

    ///
    /// Set the mesh's absolute position.
    ///
    pub fn abs_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    ///
    /// Set the mesh's position relative to it's current position.
    ///
    pub fn rel_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }
}

//
// Implementation of polygon iterators.
//
pub struct PolyIterator<'a> {
    vertex_list: &'a [prim::Vertex],
    normal_list: &'a [prim::Vector],
    polygon_list: &'a [prim::IndexPoly],
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
    type Item = prim::RefPoly<'a>;

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
                prim::RefPoly::new(
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

//
// Pipeline functions for converting the mesh from camera space to screen space
//
impl Mesh {
    ///
    /// Create a new mesh that has been run through the pipeline and contains only the polygons that should be drawn.
    ///
    pub fn run_pipeline(&self, project_mat: &prim::TransformMatrix, window_size: [f32; 2]) -> Mesh {
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
            let pi = std::f32::consts::PI;
            let x_rot = self.orientation.x * (pi / 180.0);
            let y_rot = self.orientation.y * (pi / 180.0);
            let z_rot = self.orientation.z * (pi / 180.0);

            prim::TransformMatrix::new_rotation(x_rot, y_rot, z_rot)
        };

        // Apply rotation then position to each vertex.
        for vertex in self.verticies.iter_mut() {
            *vertex = *vertex * rotation_matrix;
            *vertex = *vertex + self.position;
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

                let divisor = f32::sqrt(norm_x * norm_x + norm_y * norm_y + norm_z * norm_z);
                norm_x /= divisor;
                norm_y /= divisor;
                norm_z /= divisor;

                prim::Vector::new(norm_x, norm_y, norm_z, 0.0)
            }
        }
    }

    ///
    /// Project the mesh from camera space to NDC space by applying a projection matrix to each vertex
    ///
    pub fn project_to_ndc(&mut self, projection_matrix: &prim::TransformMatrix) {
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
    pub fn project_to_screen(&mut self, screen_width: f32, screen_height: f32) {
        let screen_width_mul = screen_width as f32 / 2.0;
        let screen_height_mul = screen_height as f32 / 2.0;
        let screen_depth_mul = 1000.0;

        for vertex in self.verticies.iter_mut() {
            vertex.x = (vertex.x + 1.0) * screen_width_mul;
            vertex.y = (vertex.y + 1.0) * screen_height_mul;
            vertex.z = screen_depth_mul - (vertex.z * screen_depth_mul);
        }
    }
}
