//! Implementations of vector methods.
//!

use super::{MatrixElement, Vector};

impl<T, const D: usize> Vector<T, D>
where
    T: MatrixElement<T> + Into<f64>,
{
    /// Return the magnitude of the vector.
    ///
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(
            self.into_iter()
                .fold(0.0, |sum, coord| sum + coord.into().powi(2)),
        )
    }
}
