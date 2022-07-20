//! Implementations of vector methods.
//!

use super::Vector;

impl<const D: usize> Vector<D> {
    /// Return the magnitude of the vector.
    ///
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.into_iter().fold(0.0, |sum, coord| sum + coord.powi(2)))
    }
}
