use std::ops::Mul;

use super::geometry::{Dim, Matrix, OrientationVector3D, Scalar, Vector};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a 4x4 matrix which can be used to represent vertex transformations.
///
#[derive(Copy, Clone)]
pub struct Matrix4X4(pub [[f64; 4]; 4]);

#[derive(Clone, Copy)]
pub struct Transform(pub(self) Matrix<4, 4>);
pub struct TransformBuilder(pub(self) Matrix<4, 4>);

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for Transform {
    /// Return an identity matrix transformation.
    ///
    fn default() -> Self {
        Self(Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }
}

impl Default for TransformBuilder {
    /// Return an identity matrix transformation.
    ///
    fn default() -> Self {
        Self(Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }
}

impl Transform {
    /// Create a new TransformBuilder which holds an identity transformation.
    ///
    #[allow(dead_code)]
    pub fn builder() -> TransformBuilder {
        TransformBuilder::default()
    }
}

impl TransformBuilder {
    /// Create a new TransformBuilder which holds an identity transformation.
    ///
    #[allow(dead_code)]
    pub fn new() -> TransformBuilder {
        TransformBuilder::default()
    }

    #[allow(dead_code)]
    pub fn build(&self) -> Transform {
        Transform(self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T> From<[[T; 4]; 4]> for Transform
where
    T: Into<f64>,
{
    fn from(array: [[T; 4]; 4]) -> Self {
        Self(Matrix::from(array))
    }
}

impl AsRef<Matrix<4, 4>> for Transform {
    fn as_ref(&self) -> &Matrix<4, 4> {
        &self.0
    }
}

impl AsRef<Matrix<4, 4>> for TransformBuilder {
    fn as_ref(&self) -> &Matrix<4, 4> {
        &self.0
    }
}

impl AsMut<Matrix<4, 4>> for Transform {
    fn as_mut(&mut self) -> &mut Matrix<4, 4> {
        &mut self.0
    }
}

impl AsMut<Matrix<4, 4>> for TransformBuilder {
    fn as_mut(&mut self) -> &mut Matrix<4, 4> {
        &mut self.0
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Matrix4X4 {
    /// Construct and return a rotation matrix
    ///
    pub fn new_rotation(rotation: OrientationVector3D) -> Matrix<4, 4> {
        let (sin_x, cos_x) = f64::sin_cos(rotation.x.to_radians());
        let (sin_y, cos_y) = f64::sin_cos(rotation.y.to_radians());
        let (sin_z, cos_z) = f64::sin_cos(rotation.z.to_radians());

        let x_rot_matrix = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_x, -sin_x, 0.0],
            [0.0, sin_x, cos_x, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let y_rot_matrix = Matrix::new([
            [cos_y, 0.0, sin_y, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_y, 0.0, cos_y, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let z_rot_matrix = Matrix::new([
            [cos_z, -sin_z, 0.0, 0.0],
            [sin_z, cos_z, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        x_rot_matrix * y_rot_matrix * z_rot_matrix
    }
}

impl TransformBuilder {
    /// Add a rotation about the x axis to the transformation.
    ///
    pub fn add_x_rotation(&self, rotation: f64) -> TransformBuilder {
        let (sin, cos) = f64::sin_cos(rotation.to_radians());
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, cos, -sin, 0.0],
                    [0.0, sin, cos, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }

    /// Add a rotation about the y axis to the transformation.
    ///
    pub fn add_y_rotation(&self, rotation: f64) -> TransformBuilder {
        let (sin, cos) = f64::sin_cos(rotation.to_radians());
        TransformBuilder(
            self.0
                * Matrix::from([
                    [cos, 0.0, sin, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-sin, 0.0, cos, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }

    /// Add a rotation about the Z axis to the transformation.
    ///
    pub fn add_z_rotation(&self, rotation: f64) -> TransformBuilder {
        let (sin, cos) = f64::sin_cos(rotation.to_radians());
        TransformBuilder(
            self.0
                * Matrix::from([
                    [cos, -sin, 0.0, 0.0],
                    [sin, cos, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }

    /// Add a 3D translation to the transformation.
    ///
    pub fn add_translation(&self, vector: Vector<3>) -> TransformBuilder {
        let (t_x, t_y, t_z) = (vector[Dim::X], vector[Dim::Y], vector[Dim::Z]);
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [t_x, t_y, t_z, 1.0],
                ]),
        )
    }

    /// Add scaling to the transformation.
    ///
    #[allow(dead_code)]
    pub fn add_scaling(&self, s: Scalar) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [s.0, 0.0, 0.0, 0.0],
                    [0.0, s.0, 0.0, 0.0],
                    [0.0, 0.0, s.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }
}

/// Operator overloads
///
impl Mul<Matrix4X4> for Matrix4X4 {
    type Output = Matrix4X4;

    fn mul(self, matrix: Matrix4X4) -> Self::Output {
        Matrix4X4([
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
