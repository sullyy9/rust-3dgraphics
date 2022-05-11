//! Implementation of a bounding box type.
//!

use std::ops::RangeInclusive;

use super::{Dim, Point};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing the limits on a specific axis.
///
// #[derive(Debug, Clone, Copy)]
// struct LimitPair {
//     min: f64,
//     max: f64,
// }

/// Type represneting a N dimensional bounding box.
///
#[derive(Debug, Clone)]
pub struct BBox<const D: usize>([RangeInclusive<f64>; D]);

pub struct BoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl BoundingBox {
    pub fn new(p1: Point<3>, p2: Point<3>) -> BoundingBox {
        let (xmin, xmax) = if p1[Dim::X] <= p2[Dim::X] {
            (p1[Dim::X], p2[Dim::X])
        } else {
            (p2[Dim::X], p1[Dim::X])
        };

        let (ymin, ymax) = if p1[Dim::Y] <= p2[Dim::Y] {
            (p1[Dim::Y], p2[Dim::Y])
        } else {
            (p2[Dim::Y], p1[Dim::Y])
        };

        let (zmin, zmax) = if p1[Dim::Z] <= p2[Dim::Z] {
            (p1[Dim::Z], p2[Dim::Z])
        } else {
            (p2[Dim::Z], p1[Dim::Z])
        };

        BoundingBox {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        }
    }
}

impl<const D: usize> Default for BBox<D> {
    fn default() -> Self {
        let range = (0.0..=0.0);
        Self([range.clone_into(target); D])
    }
}

impl<const D: usize> BBox<D> {
    /// Return a new BoundingBox given 2 points at oposite corners.
    ///
    pub fn new(p1: Point<D>, p2: Point<D>) -> BBox<D> {
        let mut bbox = BBox::default();
        bbox.0
            .iter_mut()
            .zip(p1.iter().zip(p2.iter()))
            .for_each(|(&mut range, (&c1, &c2))| {
                if c1 <= c2 {
                    range = c1..=c2;
                } else {
                    range = c2..=c1;
                }
            });
        bbox
    }
}

////////////////////////////////////////////////////////////////////////////////
// Method Implementations //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> BBox<D> {
    pub fn bounds(&self, point: Point<D>) -> bool {
        point
            .iter()
            .zip(self.0.iter())
            .all(|(coord, range)| range.contains(coord))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {}
