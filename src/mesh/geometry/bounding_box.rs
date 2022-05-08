//! Implementation of a bounding box type.
//!

use super::{Dim, Point};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing the limits on a specific axis.
struct LimitPair {
    min: f64,
    max: f64,
}

/// Type represneting a N dimensional bounding box.
///
pub struct BBox<const D: usize>([LimitPair; D]);

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
