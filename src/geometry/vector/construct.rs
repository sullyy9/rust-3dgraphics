//! Implementations of construction methods for vectors.
//!

use std::ops::Sub;

use super::{Matrix, MatrixElement, Point, Scalar, Vector};

impl<T, const D: usize> Default for Vector<T, D>
where
    T: MatrixElement<T>,
{
    /// Construct a zero length vector.
    ///
    fn default() -> Self {
        Self(Matrix::new([[T::default(); D]]))
    }
}

impl<T, const D: usize> Vector<T, D>
where
    T: MatrixElement<T>,
{
    /// Construct a vector from an array.
    ///
    pub fn new(components: [T; D]) -> Self {
        Self(Matrix::new([components.map(|comp| comp)]))
    }

    /// Construct a vector spanning two points.
    ///
    pub fn from_points(tail: Point<T, D>, head: Point<T, D>) -> Vector<T, D> {
        head.sub(tail)
    }
}

impl<const D: usize> Vector<f64, D> {
    /// Construct a vector normal to two vectors.
    ///
    pub fn normal_to(vector1: Vector<f64, D>, vector2: Vector<f64, D>) -> Vector<f64, D> {
        // Calculate the cross product of the 2 given vectors to get a vector perpendicular to
        // both.
        let mut normal_vector: Vector<f64, D> = Vector::default();
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

impl<T, const D: usize> From<Matrix<T, 1, D>> for Vector<T, D> {
    /// Construct a vector from a 1 row matrix.
    ///
    fn from(matrix: Matrix<T, 1, D>) -> Self {
        Vector(matrix)
    }
}
