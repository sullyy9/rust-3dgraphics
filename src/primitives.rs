///
/// Vertex
///
#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type Vector = Vertex;
//
// Constructor functions
//
impl Vertex {
    /// Create a new Vertex from coordinates
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vertex {
        Vertex { x, y, z, w }
    }
}
//
// Operator overloads
//
impl std::ops::Add<Vertex> for Vertex {
    type Output = Vertex;

    fn add(self, vertex: Vertex) -> Self::Output {
        Vertex {
            x: self.x + vertex.x,
            y: self.y + vertex.y,
            z: self.z + vertex.z,
            w: self.w + vertex.w,
        }
    }
}
impl std::ops::Add<f32> for Vertex {
    type Output = Vertex;

    fn add(self, value: f32) -> Self::Output {
        Vertex {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
            w: self.w + value,
        }
    }
}
impl std::ops::Sub<Vertex> for Vertex {
    type Output = Vertex;

    fn sub(self, vertex: Vertex) -> Self::Output {
        Vertex {
            x: self.x - vertex.x,
            y: self.y - vertex.y,
            z: self.z - vertex.z,
            w: self.w - vertex.w,
        }
    }
}
impl std::ops::Sub<f32> for Vertex {
    type Output = Vertex;

    fn sub(self, value: f32) -> Self::Output {
        Vertex {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
            w: self.w - value,
        }
    }
}
impl std::ops::Div<f32> for Vertex {
    type Output = Vertex;

    fn div(self, divisor: f32) -> Self::Output {
        Vertex {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            w: self.w / divisor,
        }
    }
}
impl std::ops::Mul<f32> for Vertex {
    type Output = Vertex;

    fn mul(self, scalar: f32) -> Self::Output {
        Vertex {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}
impl std::ops::Mul<TransformMatrix> for Vertex {
    type Output = Vertex;

    fn mul(self, matrix: TransformMatrix) -> Self::Output {
        Vertex {
            x: self.x * matrix.0[0][0]
                + self.y * matrix.0[1][0]
                + self.z * matrix.0[2][0]
                + self.w * matrix.0[3][0],
            y: self.x * matrix.0[0][1]
                + self.y * matrix.0[1][1]
                + self.z * matrix.0[2][1]
                + self.w * matrix.0[3][1],
            z: self.x * matrix.0[0][2]
                + self.y * matrix.0[1][2]
                + self.z * matrix.0[2][2]
                + self.w * matrix.0[3][2],
            w: self.x * matrix.0[0][3]
                + self.y * matrix.0[1][3]
                + self.z * matrix.0[2][3]
                + self.w * matrix.0[3][3],
        }
    }
}

impl Vertex {
    ///
    /// Check if a vertex is within ndc space.
    ///
    pub fn in_ndc_space(&self) -> bool {
        if (self.x.abs() < 1.0) && (self.y.abs() < 1.0) && (self.z.abs() < 1.0) {
            true
        } else {
            false
        }
    }
}

///
/// 4x4 transformation matrix
///
#[derive(Copy, Clone)]
pub struct TransformMatrix(pub [[f32; 4]; 4]);
//
// Constructor functions
//
impl TransformMatrix {
    ///
    /// Construct and return a rotation matrix
    ///
    pub fn new_rotation(x_rot: f32, y_rot: f32, z_rot: f32) -> TransformMatrix {
        let sin_x = f32::sin(x_rot);
        let cos_x = f32::cos(x_rot);
        let sin_y = f32::sin(y_rot);
        let cos_y = f32::cos(y_rot);
        let sin_z = f32::sin(z_rot);
        let cos_z = f32::cos(z_rot);

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
        z_rot_matrix * y_rot_matrix * x_rot_matrix
    }
}

//
// Operator overloads
//
impl std::ops::Mul<TransformMatrix> for TransformMatrix {
    type Output = TransformMatrix;

    fn mul(self, matrix: TransformMatrix) -> Self::Output {
        TransformMatrix([
            [
                self.0[0][0] * matrix.0[0][0]
                    + self.0[0][1] * matrix.0[1][0]
                    + self.0[0][2] * matrix.0[2][0]
                    + self.0[0][3] * matrix.0[3][0],
                self.0[0][0] * matrix.0[0][1]
                    + self.0[0][1] * matrix.0[1][1]
                    + self.0[0][2] * matrix.0[2][1]
                    + self.0[0][3] * matrix.0[3][1],
                self.0[0][0] * matrix.0[0][2]
                    + self.0[0][1] * matrix.0[1][2]
                    + self.0[0][2] * matrix.0[2][2]
                    + self.0[0][3] * matrix.0[3][2],
                self.0[0][0] * matrix.0[0][3]
                    + self.0[0][1] * matrix.0[1][3]
                    + self.0[0][2] * matrix.0[2][3]
                    + self.0[0][3] * matrix.0[3][3],
            ],
            [
                self.0[1][0] * matrix.0[0][0]
                    + self.0[1][1] * matrix.0[1][0]
                    + self.0[1][2] * matrix.0[2][0]
                    + self.0[1][3] * matrix.0[3][0],
                self.0[1][0] * matrix.0[0][1]
                    + self.0[1][1] * matrix.0[1][1]
                    + self.0[1][2] * matrix.0[2][1]
                    + self.0[1][3] * matrix.0[3][1],
                self.0[1][0] * matrix.0[0][2]
                    + self.0[1][1] * matrix.0[1][2]
                    + self.0[1][2] * matrix.0[2][2]
                    + self.0[1][3] * matrix.0[3][2],
                self.0[1][0] * matrix.0[0][3]
                    + self.0[1][1] * matrix.0[1][3]
                    + self.0[1][2] * matrix.0[2][3]
                    + self.0[1][3] * matrix.0[3][3],
            ],
            [
                self.0[2][0] * matrix.0[0][0]
                    + self.0[2][1] * matrix.0[1][0]
                    + self.0[2][2] * matrix.0[2][0]
                    + self.0[2][3] * matrix.0[3][0],
                self.0[2][0] * matrix.0[0][1]
                    + self.0[2][1] * matrix.0[1][1]
                    + self.0[2][2] * matrix.0[2][1]
                    + self.0[2][3] * matrix.0[3][1],
                self.0[2][0] * matrix.0[0][2]
                    + self.0[2][1] * matrix.0[1][2]
                    + self.0[2][2] * matrix.0[2][2]
                    + self.0[2][3] * matrix.0[3][2],
                self.0[2][0] * matrix.0[0][3]
                    + self.0[2][1] * matrix.0[1][3]
                    + self.0[2][2] * matrix.0[2][3]
                    + self.0[2][3] * matrix.0[3][3],
            ],
            [
                self.0[3][0] * matrix.0[0][0]
                    + self.0[3][1] * matrix.0[1][0]
                    + self.0[3][2] * matrix.0[2][0]
                    + self.0[3][3] * matrix.0[3][0],
                self.0[3][0] * matrix.0[0][1]
                    + self.0[3][1] * matrix.0[1][1]
                    + self.0[3][2] * matrix.0[2][1]
                    + self.0[3][3] * matrix.0[3][1],
                self.0[3][0] * matrix.0[0][2]
                    + self.0[3][1] * matrix.0[1][2]
                    + self.0[3][2] * matrix.0[2][2]
                    + self.0[3][3] * matrix.0[3][2],
                self.0[3][0] * matrix.0[0][3]
                    + self.0[3][1] * matrix.0[1][3]
                    + self.0[3][2] * matrix.0[2][3]
                    + self.0[3][3] * matrix.0[3][3],
            ],
        ])
    }
}

///
/// Polygon
///
#[derive(Copy, Clone)]
pub struct Polygon<T> {
    pub p1: T,
    pub p2: T,
    pub p3: T,

    pub normal: T,
}
impl<T> Polygon<T> {
    pub fn new(p1: T, p2: T, p3: T, normal: T) -> Polygon<T> {
        Polygon { p1, p2, p3, normal }
    }
}

/// Polygon where verticies are indexes to lists.
pub type IndexPoly = Polygon<usize>;

/// Polygons where all members are references.
pub type RefPoly<'a> = Polygon<&'a Vertex>;
