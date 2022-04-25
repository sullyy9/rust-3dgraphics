//! Implementation of operator overides for the Atomic3D type which forms the base of all geometric types.
//!

/// Macro to implement operator overloads on atomic 3D types.
/// 
/// # Examples
/// 
/// ```
/// impl_scaler_arithmetic! {impl Mul for Point3D}
/// impl_scaler_arithmetic! {impl Div for Point3D}
/// impl_scaler_arithmetic! {impl MulAssign for Point3D}
/// impl_scaler_arithmetic! {impl DivAssign for Point3D}
/// ```
///
#[macro_export]
macro_rules! impl_scaler_arithmetic {
    (impl Mul for $self_type_: ident) => {
        impl<T: Into<f64>> Mul<T> for $self_type_ {
            type Output = Self;

            fn mul(self, rhs: T) -> Self {
                let rhs = rhs.into();
                $self_type_ {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                }
            }
        }
    };
    (impl MulAssign for $self_type_: ident) => {
        impl<T: Into<f64>> MulAssign<T> for $self_type_ {
            fn mul_assign(&mut self, rhs: T) {
                let rhs = rhs.into();
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }
    };
    (impl Div for $self_type_: ident) => {
        impl<T: Into<f64>> Div<T> for $self_type_ {
            type Output = Self;

            fn div(self, rhs: T) -> Self {
                let rhs = rhs.into();
                $self_type_ {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                }
            }
        }
    };
    (impl DivAssign for $self_type_: ident) => {
        impl<T: Into<f64>> DivAssign<T> for $self_type_ {
            fn div_assign(&mut self, rhs: T) {
                let rhs = rhs.into();
                self.x /= rhs;
                self.y /= rhs;
                self.z /= rhs;
            }
        }
    }
}
