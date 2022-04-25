//! Implementation of a bounding cube type.
//!

use super::Point3D;

pub struct BoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            xmax: Default::default(),
            xmin: Default::default(),
            ymax: Default::default(),
            ymin: Default::default(),
            zmax: Default::default(),
            zmin: Default::default(),
        }
    }
}

impl BoundingBox {
    pub fn new(p1: Point3D, p2: Point3D) -> BoundingBox {
        let (xmin, xmax) = if p1.x <= p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };

        let (ymin, ymax) = if p1.y <= p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };

        let (zmin, zmax) = if p1.z <= p2.z {
            (p1.z, p2.z)
        } else {
            (p2.z, p1.z)
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
