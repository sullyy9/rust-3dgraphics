//! Implementation of a 3D orientation type.
//!

use super::angle::Angle;
use std::ops::{Index, IndexMut};

///////////////////////////////////////////////////////////////////////////////////////////////////
///Types & Traits /////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum RotationAxis {
    Roll,
    Pitch,
    Yaw,
    N(usize),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Orientation<T, const D: usize>([T; D])
where
    T: Angle;

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T, const D: usize> Orientation<T, D>
where
    T: Angle,
{
    /// Return a new Orientation, given it's angles.
    ///
    pub fn new(angles: [T; D]) -> Orientation<T, D> {
        Orientation(angles)
    }
}

impl<T, const D: usize> Index<RotationAxis> for Orientation<T, D>
where
    T: Angle,
{
    type Output = T;

    fn index(&self, index: RotationAxis) -> &Self::Output {
        match index {
            RotationAxis::Roll => &self.0[0],
            RotationAxis::Pitch => &self.0[1],
            RotationAxis::Yaw => &self.0[2],
            RotationAxis::N(i) => &self.0[i],
        }
    }
}

impl<T, const D: usize> IndexMut<RotationAxis> for Orientation<T, D>
where
    T: Angle,
{
    fn index_mut(&mut self, index: RotationAxis) -> &mut Self::Output {
        match index {
            RotationAxis::Roll => &mut self.0[0],
            RotationAxis::Pitch => &mut self.0[1],
            RotationAxis::Yaw => &mut self.0[2],
            RotationAxis::N(i) => &mut self.0[i],
        }
    }
}

impl<T, const D: usize> Default for Orientation<T, D>
where
    T: Angle + Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); D])
    }
}
