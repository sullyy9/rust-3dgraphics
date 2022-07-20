//! Scaler type
//!

/// Type representing a salar value.
///
#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Scalar(pub f64);

impl<T> From<T> for Scalar
where
    T: Into<f64>,
{
    /// Construct a new scalar by converting a type.
    ///
    fn from(val: T) -> Self {
        Scalar(val.into())
    }
}