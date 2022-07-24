use crate::geometry::Radians;

use super::geometry::{Dim, Matrix, Scalar, Vector};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Transform(pub(self) Matrix<f64, 4, 4>);
pub struct TransformBuilder(pub(self) Matrix<f64, 4, 4>);

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
    /// Create a new TransformBuilder which contains the base of an affine transformation.
    ///
    #[allow(dead_code)]
    pub fn builder() -> TransformBuilder {
        TransformBuilder::new()
    }
}

impl TransformBuilder {
    /// Create a new TransformBuilder which contains the base of an affine transformation.
    ///
    #[allow(dead_code)]
    pub fn new() -> TransformBuilder {
        Self(Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]))
    }

    #[allow(dead_code)]
    pub fn build_affine(&self) -> Transform {
        Transform(self.0)
    }

    #[allow(dead_code)]
    pub fn build_projection(&self) -> Transform {
        let mut transform = Transform(self.0);
        transform.0[2][3] = 1.0;
        transform.0[3][3] = 0.0;
        transform
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl From<[[f64; 4]; 4]> for Transform {
    fn from(array: [[f64; 4]; 4]) -> Self {
        Self(Matrix::from(array))
    }
}

impl AsRef<Matrix<f64, 4, 4>> for Transform {
    fn as_ref(&self) -> &Matrix<f64, 4, 4> {
        &self.0
    }
}

impl AsRef<Matrix<f64, 4, 4>> for TransformBuilder {
    fn as_ref(&self) -> &Matrix<f64, 4, 4> {
        &self.0
    }
}

impl AsMut<Matrix<f64, 4, 4>> for Transform {
    fn as_mut(&mut self) -> &mut Matrix<f64, 4, 4> {
        &mut self.0
    }
}

impl AsMut<Matrix<f64, 4, 4>> for TransformBuilder {
    fn as_mut(&mut self) -> &mut Matrix<f64, 4, 4> {
        &mut self.0
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl TransformBuilder {
    /// Add a rotation about the x axis to the transformation.
    ///
    pub fn rotate_about_x<T>(&self, angle: T) -> TransformBuilder
    where
        T: Into<Radians>,
    {
        let (sin, cos) = f64::sin_cos(angle.into().0);
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
    pub fn rotate_about_y<T>(&self, angle: T) -> TransformBuilder
    where
        T: Into<Radians>,
    {
        let (sin, cos) = f64::sin_cos(angle.into().0);
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
    pub fn rotate_about_z<T>(&self, angle: T) -> TransformBuilder
    where
        T: Into<Radians>,
    {
        let (sin, cos) = f64::sin_cos(angle.into().0);
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

    /// Add a translation to a transformation.
    ///
    pub fn translate(&self, vector: Vector<f64, 3>) -> TransformBuilder {
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

    /// Add a translation along the X axis to a transformation.
    ///
    #[allow(dead_code)]
    pub fn translate_x(&self, t_x: f64) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [t_x, 0.0, 0.0, 1.0],
                ]),
        )
    }

    /// Add a translation along the X axis to a transformation.
    ///
    #[allow(dead_code)]
    pub fn translate_y(&self, t_y: f64) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, t_y, 0.0, 1.0],
                ]),
        )
    }

    /// Add a translation along the X axis to a transformation.
    ///
    #[allow(dead_code)]
    pub fn translate_z(&self, t_z: f64) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, t_z, 1.0],
                ]),
        )
    }

    /// Add scaling to each axis to a transformation.
    ///
    #[allow(dead_code)]
    pub fn scale(&self, s: Scalar<f64>) -> TransformBuilder {
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

    /// Add X axis scaling to a transformation.
    ///
    #[allow(dead_code)]
    pub fn scale_x(&self, s: Scalar<f64>) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [s.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }

    /// Add Y axis scaling to a transformation.
    ///
    #[allow(dead_code)]
    pub fn scale_y(&self, s: Scalar<f64>) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, s.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }

    /// Add Z axis scaling to a transformation.
    ///
    #[allow(dead_code)]
    pub fn scale_z(&self, s: Scalar<f64>) -> TransformBuilder {
        TransformBuilder(
            self.0
                * Matrix::from([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, s.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]),
        )
    }
}
