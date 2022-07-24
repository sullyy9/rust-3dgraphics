//! Implementation of a generic Matrix type.
//!

mod addition;
mod divisision;
mod mat;
mod multiplication;
mod subtraction;

// Internal re-exports for types required by sub-modules
pub(self) use super::Scalar;

// External re-exports.
pub use self::mat::{Matrix, MatrixElement};
