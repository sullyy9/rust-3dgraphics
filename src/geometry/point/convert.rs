//! Implementations of traits and methods for point type conversion.
//!

use crate::geometry::Dim;

use super::{Matrix, MatrixElement, Point};

/// Point -> Point
///
impl<T, const D: usize> AsRef<Point<T, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_ref(&self) -> &Point<T, D> {
        self
    }
}
impl<T, const D: usize> AsMut<Point<T, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_mut(&mut self) -> &mut Point<T, D> {
        self
    }
}

impl<T> Point<T, 3>
where
    T: MatrixElement<T> + From<u8>,
{
    pub fn to_homogenous(self) -> Point<T, 4> {
        use Dim::{X, Y, Z};

        Point::new([self[X], self[Y], self[Z], 1.into()])
    }

    pub fn from_homogenous(point: Point<T, 4>) -> Self {
        use Dim::{W, X, Y, Z};

        let x = point[X] / point[W];
        let y = point[Y] / point[W];
        let z = point[Z] / point[W];
        Self::new([x, y, z])
    }
}

/// Point -> Matrix
///
impl<T, const D: usize> AsRef<Matrix<T, 1, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_ref(&self) -> &Matrix<T, 1, D> {
        &self.0
    }
}
impl<T, const D: usize> AsMut<Matrix<T, 1, D>> for Point<T, D>
where
    T: MatrixElement<T>,
{
    fn as_mut(&mut self) -> &mut Matrix<T, 1, D> {
        &mut self.0
    }
}

/// Conversion between primitive types.
///
macro_rules! prim_convert_impl {
    ($($from_t:ty),+ => $into_t:ty) => {$(
        impl<const D: usize> From<Point<$from_t, D>> for Point<$into_t, D>
        where
            $from_t: MatrixElement<$from_t>,
            $into_t: MatrixElement<$into_t>,
        {
            fn from(point: Point<$from_t, D>) -> Self {
                point.map(|i| i.into())
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
        impl<const D: usize> TryFrom<Point<$from_t, D>> for Point<$into_t, D>
        where
            $from_t: MatrixElement<$from_t>,
            $into_t: MatrixElement<$into_t>,
        {
            type Error = std::num::TryFromIntError;

            fn try_from(point: Point<$from_t, D>) -> Result<Self, Self::Error> {
                let mut error = None;
                let new_point = point.map(|i| {
                    i.try_into().unwrap_or_else(|e| {
                        error = Some(e);
                        Default::default()
                    })
                });

                error.map_or(Ok(new_point), Err)
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
