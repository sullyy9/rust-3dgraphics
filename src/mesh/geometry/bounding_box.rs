//! Implementation of a bounding box type.
//!

use super::Point;

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type represneting a N dimensional bounding box.
///
#[derive(Debug, Clone)]
pub struct BBox<const D: usize>([(f64, f64); D]);

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Default for BBox<D> {
    fn default() -> Self {
        Self([(0.0, 0.0); D])
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
            .for_each(|((min, max), (&c1, &c2))| {
                if c1 <= c2 {
                    (*min, *max) = (c1, c2);
                } else {
                    (*min, *max) = (c2, c1);
                }
            });
        bbox
    }
}

////////////////////////////////////////////////////////////////////////////////
// Method Implementations //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> BBox<D> {
    pub fn bounds(&self, point: &Point<D>) -> bool {
        point
            .iter()
            .zip(self.0.iter())
            .all(|(coord, &(min, max))| (min..=max).contains(coord))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box() {
        let bbox = BBox::new(
            Point::new([0.44, 50.28, -88.62, -0.24]),
            Point::new([60, 100, -20, 0]),
        );

        let point_bound = Point::new([32.6, 50.29, -50.3, -0.1]);
        let point_not_bound = Point::new([32.6, 0.0, -50.3, -0.1]);

        assert!(bbox.bounds(&point_bound));
        assert!(!bbox.bounds(&point_not_bound));
    }
}
