//! Implementation of methods to transform the mesh structure between different spaces.
//! 

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