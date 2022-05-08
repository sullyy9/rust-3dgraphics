//! This module contains traits that define the shared behaviour between points and vectors of various dimensions.
//!

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub enum Dim {
    X,
    Y,
    Z,
    W,
}

////////////////////////////////////////////////////////////////////////////////
// Macros //////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Implement indexing via the Coord enum to type up to 4 dimensions.
/// The type must be a tuple struct where the inner type is an array of f64's. The first element
/// will be indexed as X, the second Y, etc.
///
/// # Examples
///
/// ```
/// impl_coord_index! {impl Index for 1D type Point1D}
/// impl_coord_index! {impl Index for 2D type Point2D}
/// impl_coord_index! {impl Index for 3D type Vector3D}
/// impl_coord_index! {impl Index for 4D type Vector4D}
/// ```
///
#[macro_export]
macro_rules! impl_coord_index {
    (impl Index for 1D type $type_: ident) => {
        impl Index<Dim> for $type_ {
            type Output = f64;

            fn index(&self, index: Dim) -> &Self::Output {
                match index {
                    Dim::X => &self.0[0],
                    _ => panic!(),
                }
            }
        }
        impl IndexMut<Dim> for $type_ {
            fn index_mut(&mut self, index: Dim) -> &mut Self::Output {
                match index {
                    Dim::X => &mut self.0[0],
                    _ => panic!(),
                }
            }
        }
    };
    (impl Index for 2D type $type_: ident) => {
        impl Index<Dim> for $type_ {
            type Output = f64;

            fn index(&self, index: Dim) -> &Self::Output {
                match index {
                    Dim::X => &self.0[0],
                    Dim::Y => &self.0[1],
                    _ => panic!(),
                }
            }
        }
        impl IndexMut<Dim> for $type_ {
            fn index_mut(&mut self, index: Dim) -> &mut Self::Output {
                match index {
                    Dim::X => &mut self.0[0],
                    Dim::Y => &mut self.0[1],
                    _ => panic!(),
                }
            }
        }
    };
    (impl Index for 3D type $type_: ident) => {
        impl Index<Dim> for $type_ {
            type Output = f64;

            fn index(&self, index: Dim) -> &Self::Output {
                match index {
                    Dim::X => &self.0[0],
                    Dim::Y => &self.0[1],
                    Dim::Z => &self.0[2],
                    _ => panic!(),
                }
            }
        }
        impl IndexMut<Dim> for $type_ {
            fn index_mut(&mut self, index: Dim) -> &mut Self::Output {
                match index {
                    Dim::X => &mut self.0[0],
                    Dim::Y => &mut self.0[1],
                    Dim::Z => &mut self.0[2],
                    _ => panic!(),
                }
            }
        }
    };
    (impl Index for 4D type $type_: ident) => {
        impl Index<Dim> for $type_ {
            type Output = f64;

            fn index(&self, index: Dim) -> &Self::Output {
                match index {
                    Dim::X => &self.0[0],
                    Dim::Y => &self.0[1],
                    Dim::Z => &self.0[2],
                    Dim::W => &self.0[3],
                }
            }
        }
        impl IndexMut<Dim> for $type_ {
            fn index_mut(&mut self, index: Dim) -> &mut Self::Output {
                match index {
                    Dim::X => &mut self.0[0],
                    Dim::Y => &mut self.0[1],
                    Dim::Z => &mut self.0[2],
                    Dim::W => &mut self.0[3],
                }
            }
        }
    };
}
