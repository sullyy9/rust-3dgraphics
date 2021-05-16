///////////////////////////////////////////////////////////////////////////////
// Vertex                                                                    //
///////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type Vector = Vertex;
impl Vertex {
    /// Create a new Vertex from coordinates
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vertex {
        Vertex { x, y, z, w }
    }
}
// Implement addition for Vertex's.
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
// Implement subtraction for Vertex's.
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
// Implement element wise division for Vertex's.
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
// Implement element wise multiplication for Vertex's.
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
// Implement matrix multiplication for Vertex's.
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

///////////////////////////////////////////////////////////////////////////////
// Transformation Matrix                                                     //
///////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub struct TransformMatrix(pub [[f32; 4]; 4]);
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

///////////////////////////////////////////////////////////////////////////////
// Polygon                                                                   //
///////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub struct Polygon<T> {
    pub p1: T,
    pub p2: T,
    pub p3: T,

    pub normal: Vector,
}
impl<T> Polygon<T> {
    pub fn new(p1: T, p2: T, p3: T, normal: Vector) -> Polygon<T> {
        Polygon {
            p1,
            p2,
            p3,
            normal,
        }
    }
}

/// Polygon where verticies are indexes to a list.
pub type IndexPolygon = Polygon<usize>;

/// Polygon where all members are references.
pub type RefPolygon<'a> = Polygon<&'a Vertex>;

/// Polygon where all members are owned.
pub type OwnPolygon = Polygon<Vertex>;