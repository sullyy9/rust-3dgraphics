//! Implementations of traits and methods for vector type conversion.
//!

use super::{Matrix, MatrixElement, Vector};

impl<T, const D: usize> AsRef<Vector<T, D>> for Vector<T, D> {
    fn as_ref(&self) -> &Vector<T, D> {
        self
    }
}
impl<T, const D: usize> AsMut<Vector<T, D>> for Vector<T, D> {
    fn as_mut(&mut self) -> &mut Vector<T, D> {
        self
    }
}

impl<T, const D: usize> Vector<T, D>
where
    T: MatrixElement<T>,
{
    /// Promote a vector to a higher dimentional vector where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Vector<T, ND> {
        let mut new_vector = Vector::default();
        let len = self.0[0].len();
        new_vector.0[0][..len].clone_from_slice(&self.0[0]);
        new_vector
    }

    /// Demote a vector to a lower dimentional vector.
    ///
    pub fn demote<const ND: usize>(&self) -> Vector<T, ND> {
        let mut new_vector = Vector::default();
        let len = new_vector.0[0].len();
        new_vector.0[0].clone_from_slice(&self.0[0][..len]);
        new_vector
    }
}

impl<T, const D: usize> AsRef<Matrix<T, 1, D>> for Vector<T, D> {
    fn as_ref(&self) -> &Matrix<T, 1, D> {
        &self.0
    }
}
impl<T, const D: usize> AsMut<Matrix<T, 1, D>> for Vector<T, D> {
    fn as_mut(&mut self) -> &mut Matrix<T, 1, D> {
        &mut self.0
    }
}

/// Conversion between primitive types.
///
macro_rules! prim_convert_impl {
    ($($from_t:ty),+ => $into_t:ty) => {$(
        impl<const D: usize> From<Vector<$from_t, D>> for Vector<$into_t, D>
        where
            $from_t: MatrixElement<$from_t>,
            $into_t: MatrixElement<$into_t>,
        {
            fn from(vector: Vector<$from_t, D>) -> Self {
                vector.map(|i| i.into())
            }
        })+
    };
}

prim_convert_impl! {i8, u8 => i16}
prim_convert_impl! {i8, u8, i16, u16 => i32}
prim_convert_impl! {i8, u8, i16, u16, i32, u32 => i64}
prim_convert_impl! {i8, u8, i16, u16, i32, u32, i64, u64 => i128}
prim_convert_impl! {i8, u8, i16 => isize}

prim_convert_impl! {u8 => u16}
prim_convert_impl! {u8, u16 => u32}
prim_convert_impl! {u8, u16, u32 => u64}
prim_convert_impl! {u8, u16, u32, u64 => u128}
prim_convert_impl! {u8, u16 => usize}

prim_convert_impl! {i8, u8, i16, u16 => f32}
prim_convert_impl! {i8, u8, i16, u16, i32, u32, f32 => f64}

macro_rules! prim_try_convert_impl {
    ($($from_t:ty),+ => $into_t:ty) => {$(
        impl<const D: usize> TryFrom<Vector<$from_t, D>> for Vector<$into_t, D>
        where
            $from_t: MatrixElement<$from_t>,
            $into_t: MatrixElement<$into_t>,
        {
            type Error = std::num::TryFromIntError;

            fn try_from(vector: Vector<$from_t, D>) -> Result<Self, Self::Error> {
                let mut error = None;
                let new_vector = vector.map(|i| {
                    i.try_into().unwrap_or_else(|e| {
                        error = Some(e);
                        Default::default()
                    })
                });

                error.map_or(Ok(new_vector), Err)
            }
        })+
    };
}

prim_try_convert_impl! {u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize => i8}
prim_try_convert_impl! {u16, i32, u32, i64, u64, i128, u128, isize, usize => i16}
prim_try_convert_impl! {u16, i32, u32, i64, u64, i128, u128, usize => isize}
prim_try_convert_impl! {u32, i64, u64, i128, u128, isize, usize => i32}
prim_try_convert_impl! {u64, i128, u128, isize, usize => i64}
prim_try_convert_impl! {u128, isize, usize => i128}

prim_try_convert_impl! {i8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize => u8}
prim_try_convert_impl! {i8, i16, i32, u32, i64, u64, i128, u128, isize, usize => u16}
prim_try_convert_impl! {i8, i16, i32, u32, i64, u64, i128, u128, isize => usize}
prim_try_convert_impl! {i8, i16, i32, i64, u64, i128, u128, isize, usize => u32}
prim_try_convert_impl! {i8, i16, i32, i64, i128, u128, isize, usize => u64}
prim_try_convert_impl! {i8, i16, i32, i64, i128, isize, usize => u128}