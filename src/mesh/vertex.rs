use super::{
    geometry::{Dim, Point},
    matrix::Matrix4X4,
};

use std::ops::Mul;

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub type Vertex = Point<4>;

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Mul<Matrix4X4> for Vertex {
    type Output = Vertex;

    fn mul(self, matrix: Matrix4X4) -> Self::Output {
        Self([
            self[Dim::X] * matrix.0[0][0]
                + self[Dim::Y] * matrix.0[1][0]
                + self[Dim::Z] * matrix.0[2][0]
                + self[Dim::W] * matrix.0[3][0],
            self[Dim::X] * matrix.0[0][1]
                + self[Dim::Y] * matrix.0[1][1]
                + self[Dim::Z] * matrix.0[2][1]
                + self[Dim::W] * matrix.0[3][1],
            self[Dim::X] * matrix.0[0][2]
                + self[Dim::Y] * matrix.0[1][2]
                + self[Dim::Z] * matrix.0[2][2]
                + self[Dim::W] * matrix.0[3][2],
            self[Dim::X] * matrix.0[0][3]
                + self[Dim::Y] * matrix.0[1][3]
                + self[Dim::Z] * matrix.0[2][3]
                + self[Dim::W] * matrix.0[3][3],
        ])
    }
}
