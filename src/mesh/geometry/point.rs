//! Implementation of an N dimensional point type.
//!

mod arith;
mod construct;
mod convert;
mod index;
mod iter;
mod method;

// Internal re-exports for types required by sub-modules
pub(self) use super::{Dim, Matrix, Vector, Scalar};

/// Type representing an N dimensional point.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point<const N: usize>(Matrix<1, N>);

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaler_mul() {
        let control_point = Point::new([0.44, 50.28, -88.62, -0.24]);
        let mut test_point = Point::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(test_point * Scalar(2.0), control_point);

        test_point *= Scalar(2.0);
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_scaler_div() {
        let control_point = Point::new([0.22, 25.14, -44.31, -0.12]);
        let mut test_point = Point::new([0.44, 50.28, -88.62, -0.24]);

        assert_eq!(test_point / Scalar(2.0), control_point);

        test_point /= Scalar(2.0);
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_vector_addition() {
        let control_point = Point::new([0.44, 50.28, -88.62, -0.24]);
        let vector = Vector::new([0.22, 25.14, -44.31, -0.12]);
        let mut test_point = Point::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(test_point + vector, control_point);

        test_point.translate(&vector);
        assert_eq!(test_point, control_point);
    }

    #[test]
    fn test_point_subtraction() {
        let point1 = Point::new([0.22, 25.14, -44.31, -0.12]);
        let point2 = Point::new([0.44, 50.28, -88.62, -0.24]);
        let vector = Vector::new([0.22, 25.14, -44.31, -0.12]);

        assert_eq!(point1.vector_to(&point2), vector);
        assert_eq!(point1.vector_from(&point2), -vector);
    }
}
