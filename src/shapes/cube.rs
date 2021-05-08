use crate::shapes::primitives::{TransformMatrix, Triangle, Vertex};
pub struct Cube {
    pub faces: [Triangle; 12],
    pub edge_length: f32,
    pub position: Vertex,
    pub orientation: Vertex,
}
impl Cube {
    /// Create a new cube.
    pub fn new(edge_length: f32, position: Vertex, orientation: Vertex) -> Cube {
        // Create the verticies of the cube with the centre at the origin. Make sure w is 1.
        let vertex_pos = edge_length / 2.0;
        let vertex_neg = -edge_length / 2.0;

        let north_west_bottom = Vertex::new(vertex_neg, vertex_neg, vertex_pos, 0.0);
        let north_east_bottom = Vertex::new(vertex_pos, vertex_neg, vertex_pos, 0.0);
        let south_west_bottom = Vertex::new(vertex_neg, vertex_neg, vertex_neg, 0.0);
        let south_east_bottom = Vertex::new(vertex_pos, vertex_neg, vertex_neg, 0.0);

        let north_west_top = Vertex::new(vertex_neg, vertex_pos, vertex_pos, 0.0);
        let north_east_top = Vertex::new(vertex_pos, vertex_pos, vertex_pos, 0.0);
        let south_west_top = Vertex::new(vertex_neg, vertex_pos, vertex_neg, 0.0);
        let south_east_top = Vertex::new(vertex_pos, vertex_pos, vertex_neg, 0.0);

        // Construct the cube from triangles
        Cube {
            faces: [
                // South face
                Triangle::new(south_west_bottom, south_west_top, south_east_top),
                Triangle::new(south_west_bottom, south_east_top, south_east_bottom),
                // East face
                Triangle::new(south_east_bottom, south_east_top, north_east_top),
                Triangle::new(south_east_bottom, north_east_top, north_east_bottom),
                // North face
                Triangle::new(north_east_bottom, north_east_top, north_west_top),
                Triangle::new(north_east_bottom, north_west_top, north_west_bottom),
                // West face
                Triangle::new(north_west_bottom, north_west_top, south_west_top),
                Triangle::new(north_west_bottom, south_west_top, south_west_bottom),
                // Top face
                Triangle::new(south_west_top, north_west_top, north_east_top),
                Triangle::new(south_west_top, north_east_top, south_east_top),
                // Bottom Face
                Triangle::new(north_west_bottom, south_west_bottom, south_east_bottom),
                Triangle::new(north_west_bottom, south_east_bottom, north_east_bottom),
            ],
            position,
            edge_length,
            orientation,
        }
    }

    /// Perform a rotation transformation given the desired orientation
    pub fn rotate(&mut self, orientation: Vertex) {
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

        let x_rot_matrix = TransformMatrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_x, -sin_x, 0.0],
            [0.0, sin_x, cos_x, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let y_rot_matrix = TransformMatrix([
            [cos_y, 0.0, sin_y, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_y, 0.0, cos_y, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let z_rot_matrix = TransformMatrix([
            [cos_z, -sin_z, 0.0, 0.0],
            [sin_z, cos_z, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let combined_matrix = z_rot_matrix * y_rot_matrix * x_rot_matrix;

        for face in self.faces.iter_mut() {
            face.p1 = face.p1 * combined_matrix;
            face.p2 = face.p2 * combined_matrix;
            face.p3 = face.p3 * combined_matrix;
        }
    }
}
