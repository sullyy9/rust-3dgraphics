//! Implementations of construction methods for vectors.
//!

use std::ops::Sub;

use super::{Matrix, Point, Scalar, Vector};

impl<const D: usize> Default for Vector<D> {
    /// Construct a zero length vector.
    ///
    fn default() -> Self {
        Self(Matrix::new([[0.0; D]]))
    }
}

impl<const D: usize> Vector<D> {
    /// Construct a vector from an array.
    ///
    pub fn new<T>(components: [T; D]) -> Self
    where
        T: Into<f64>,
    {
        Self(Matrix::new([components.map(|comp| comp.into())]))
    }

    /// Construct a vector spanning two points.
    ///
    pub fn from_points(tail: Point<D>, head: Point<D>) -> Vector<D> {
        head.sub(tail)
    }

    /// Construct a vector normal to two vectors.
    ///
    pub fn normal_to(vector1: Vector<D>, vector2: Vector<D>) -> Vector<D> {
        // Calculate the cross product of the 2 given vectors to get a vector perpendicular to
        // both.
        let mut normal_vector: Vector<D> = Vector::default();
        normal_vector.0[0][0] =
            (vector1.0[0][1] * vector2.0[0][2]) - (vector1.0[0][2] * vector2.0[0][1]);
        normal_vector.0[0][1] =
            (vector1.0[0][2] * vector2.0[0][0]) - (vector1.0[0][0] * vector2.0[0][2]);
        normal_vector.0[0][2] =
            (vector1.0[0][0] * vector2.0[0][1]) - (vector1.0[0][1] * vector2.0[0][0]);

        // Normalise the vector (It's magnitude should be 1).
        normal_vector /= Scalar(f64::sqrt(
            normal_vector.0[0][0].powi(2)
                + normal_vector.0[0][1].powi(2)
                + normal_vector.0[0][2].powi(2),
        ));
        normal_vector
    }
}

impl<const D: usize> From<Matrix<1, D>> for Vector<D> {
    /// Construct a vector from a 1 row matrix.
    ///
    fn from(matrix: Matrix<1, D>) -> Self {
        Vector(matrix)
    }
}
