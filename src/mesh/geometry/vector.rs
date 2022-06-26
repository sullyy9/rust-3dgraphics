//! Implementation of an N dimensional vector type.
//!

mod arith;
mod construct;
mod convert;
mod index;
mod iter;
mod method;

pub(self) use super::{Dim, Matrix, Point};

/// Type representing an N dimensional vector.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub Matrix<1, N>);

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaler_mul() {
        let coords = [0.43, 56.28, -87.52, -0.23];
        let scaler = 4.87;

        let coords_scaled = coords.map(|coord| coord * 4.87);

        assert_eq!(Vector::new(coords) * scaler, Vector::new(coords_scaled));

        let mut point_mul_assign = Vector::new(coords);
        point_mul_assign *= scaler;
        assert_eq!(point_mul_assign, Vector::new(coords_scaled));
    }

    #[test]
    fn test_scaler_div() {
        let coords = [0.43, 56.28, -87.52, -0.23];
        let scaler = 4.87;

        let coords_scaled = coords.map(|coord| coord / 4.87);

        assert_eq!(Vector::new(coords) / scaler, Vector::new(coords_scaled));

        let mut point_mul_assign = Vector::new(coords);
        point_mul_assign /= scaler;
        assert_eq!(point_mul_assign, Vector::new(coords_scaled));
    }
}
