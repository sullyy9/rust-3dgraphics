use crate::matrix;

///
/// Vertex
///
#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
pub type Vector = Vertex;
//
// Constructor functions
//
impl Vertex {
    /// Create a new Vertex from coordinates
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vertex {
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
impl std::ops::Add<f64> for Vertex {
    type Output = Vertex;

    fn add(self, value: f64) -> Self::Output {
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
impl std::ops::Sub<f64> for Vertex {
    type Output = Vertex;

    fn sub(self, value: f64) -> Self::Output {
        Vertex {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
            w: self.w - value,
        }
    }
}
impl std::ops::Div<f64> for Vertex {
    type Output = Vertex;

    fn div(self, divisor: f64) -> Self::Output {
        Vertex {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            w: self.w / divisor,
        }
    }
}
impl std::ops::Mul<f64> for Vertex {
    type Output = Vertex;

    fn mul(self, scalar: f64) -> Self::Output {
        Vertex {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}
impl std::ops::Mul<matrix::TransformMatrix> for Vertex {
    type Output = Vertex;

    fn mul(self, matrix: matrix::TransformMatrix) -> Self::Output {
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