//! Scaler type
//!

/// Type representing a salar value.
///
#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Scalar<T>(pub T);

impl<T> From<T> for Scalar<T> {
    /// Construct a new scalar by converting a type.
    ///
    fn from(val: T) -> Self {
        Scalar(val)
    }
}
