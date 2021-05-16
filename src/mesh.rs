use crate::primitives as prim;

/// The mesh consists of a number of verticies and polygons. A polygon is simply 3 indicies
/// to the positions its verticies in the verticies vector.
#[derive(Clone)]
pub struct Mesh {
    pub verticies: Vec<prim::Vertex>,
    pub polygons: Vec<prim::IndexPolygon>,

    pub visible_polygons: Vec<prim::IndexPolygon>,

    pub position: prim::Vertex,
    pub orientation: prim::Vertex,
}
/// Construction functions
impl Mesh {
    /// Create an empty mesh
    pub fn new() -> Mesh {
        let verticies = Vec::new();
        let polygons = Vec::new();
        let visible_polygons = Vec::new();

        let position = prim::Vertex::new(0.0, 0.0, 0.0, 0.0);
        let orientation = prim::Vertex::new(0.0, 0.0, 0.0, 0.0);

        Mesh {
            verticies,
            polygons,
            visible_polygons,
            position,
            orientation,
        }
    }

    /// Load a cube into the mesh
    pub fn load_cube(&mut self, edge_length: f32) {
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        self.verticies.push(prim::Vertex::new(neg, neg, pos, 0.0));
        self.verticies.push(prim::Vertex::new(pos, neg, pos, 0.0));
        self.verticies.push(prim::Vertex::new(neg, neg, neg, 0.0));
        self.verticies.push(prim::Vertex::new(pos, neg, neg, 0.0));
        self.verticies.push(prim::Vertex::new(neg, pos, pos, 0.0));
        self.verticies.push(prim::Vertex::new(pos, pos, pos, 0.0));
        self.verticies.push(prim::Vertex::new(neg, pos, neg, 0.0));
        self.verticies.push(prim::Vertex::new(pos, pos, neg, 0.0));

        let norm = prim::Vector::new(0.0, 0.0, 0.0, 0.0);
        self.polygons.push(prim::Polygon::new(2, 6, 7, norm));
        self.polygons.push(prim::Polygon::new(2, 7, 3, norm));
        self.polygons.push(prim::Polygon::new(3, 7, 5, norm));
        self.polygons.push(prim::Polygon::new(3, 5, 1, norm));
        self.polygons.push(prim::Polygon::new(1, 5, 4, norm));
        self.polygons.push(prim::Polygon::new(1, 4, 0, norm));
        self.polygons.push(prim::Polygon::new(0, 4, 6, norm));
        self.polygons.push(prim::Polygon::new(0, 6, 2, norm));
        self.polygons.push(prim::Polygon::new(6, 4, 5, norm));
        self.polygons.push(prim::Polygon::new(6, 5, 7, norm));
        self.polygons.push(prim::Polygon::new(0, 2, 3, norm));
        self.polygons.push(prim::Polygon::new(0, 3, 1, norm));
    }
}

/// Transformation functions
impl Mesh {
    /// Perform a rotation transformation given the desired orientation
    pub fn rotate(&mut self, orientation: prim::Vertex) {
        let pi = std::f32::consts::PI;
        let x_rotation = (orientation.x - self.orientation.x) * (pi / 180.0);
        let y_rotation = (orientation.y - self.orientation.y) * (pi / 180.0);
        let z_rotation = (orientation.z - self.orientation.z) * (pi / 180.0);
        self.orientation = orientation;

        let sin_x = f32::sin(x_rotation);
        let cos_x = f32::cos(x_rotation);
        let sin_y = f32::sin(y_rotation);
        let cos_y = f32::cos(y_rotation);
        let sin_z = f32::sin(z_rotation);
        let cos_z = f32::cos(z_rotation);

        let x_rot_matrix = prim::TransformMatrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_x, -sin_x, 0.0],
            [0.0, sin_x, cos_x, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let y_rot_matrix = prim::TransformMatrix([
            [cos_y, 0.0, sin_y, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_y, 0.0, cos_y, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let z_rot_matrix = prim::TransformMatrix([
            [cos_z, -sin_z, 0.0, 0.0],
            [sin_z, cos_z, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let combined_matrix = z_rot_matrix * y_rot_matrix * x_rot_matrix;

        for vertex in self.verticies.iter_mut() {
            *vertex = *vertex * combined_matrix;
        }
    }

    /// Take an index polygon
    /// Return a polygon consisting of references to it's verticies.
    pub fn get_polygon_ref(&self, polygon: &prim::IndexPolygon) -> prim::RefPolygon {
        let p1 = &self.verticies[polygon.p1];
        let p2 = &self.verticies[polygon.p2];
        let p3 = &self.verticies[polygon.p3];
        let normal = polygon.normal;

        prim::Polygon::new(p1, p2, p3, normal)
    }

    /// Take an index polygon
    /// Return a polygon which owns it's verticies.
    pub fn get_polygon_owned(&self, polygon: &prim::IndexPolygon) -> prim::OwnPolygon {
        let p1 = self.verticies[polygon.p1];
        let p2 = self.verticies[polygon.p2];
        let p3 = self.verticies[polygon.p3];
        let normal = polygon.normal;

        prim::Polygon::new(p1, p2, p3, normal)
    }
}

/// Pipeline functions
impl Mesh {
    pub fn find_normals(&mut self) {
        for i in 0..self.polygons.len() {
            let index_polygon = self.polygons[i];
            let polygon = self.get_polygon_ref(&index_polygon);

            let vector_a = *polygon.p2 - *polygon.p1;
            let vector_b = *polygon.p3 - *polygon.p1;

            let mut normal_x = vector_a.y * vector_b.z - vector_a.z * vector_b.y;
            let mut normal_y = vector_a.z * vector_b.x - vector_a.x * vector_b.z;
            let mut normal_z = vector_a.x * vector_b.y - vector_a.y * vector_b.x;

            // Make the normal a unit vector
            let divisor =
                f32::sqrt(normal_x * normal_x + normal_y * normal_y + normal_z * normal_z);

            normal_x = normal_x / divisor;
            normal_y = normal_y / divisor;
            normal_z = normal_z / divisor;

            self.polygons[i].normal = prim::Vector::new(normal_x, normal_y, normal_z, 0.0);
        }
    }

    /// Project a mesh object from camera space to NDC space.
    pub fn project_to_ndc(&mut self, projection_matrix: &prim::TransformMatrix) {
        for vertex in self.verticies.iter_mut() {
            *vertex = *vertex + self.position;
            *vertex = *vertex * (*projection_matrix);
            *vertex = *vertex / vertex.w;
        }
    }

    /// Copy any polygons that are at least partially within ndc space, into the polygon list.
    pub fn polygons_in_view(&mut self) {
        for index_polygon in self.polygons.iter() {
            let polygon: prim::RefPolygon = self.get_polygon_ref(index_polygon);

            if (polygon.p1.x.abs() < 1.0 && polygon.p1.y.abs() < 1.0 && polygon.p1.z.abs() < 1.0)
                || (polygon.p2.x.abs() < 1.0
                    && polygon.p2.y.abs() < 1.0
                    && polygon.p2.z.abs() < 1.0)
                || (polygon.p3.x.abs() < 1.0
                    && polygon.p3.y.abs() < 1.0
                    && polygon.p3.z.abs() < 1.0)
            {
                self.visible_polygons.push(index_polygon.to_owned());
            }
        }
    }

    /// Take a mesh and Transform all of its points to screen space.
    pub fn project_to_screen(&mut self, screen_width: f32, screen_height: f32) {
        let screen_width_mul = screen_width as f32 / 2.0;
        let screen_height_mul = screen_height as f32 / 2.0;
        let screen_depth_mul = 255.0;
        for vertex in self.verticies.iter_mut() {
            vertex.x = (vertex.x + 1.0) * screen_width_mul;
            vertex.y = (vertex.y + 1.0) * screen_height_mul;
            vertex.z = vertex.z * screen_depth_mul;
            vertex.z = 255.0 - vertex.z; // We want z to be higher the closer to the camera it is
        }
    }
}
