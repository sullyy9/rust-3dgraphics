//! Implementation of a bounding box type.
//!

use super::{MatrixElement, Point};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type represneting a N dimensional bounding box.
///
#[derive(Debug, Clone)]
pub struct BBox<T, const D: usize>([(T, T); D])
where
    T: MatrixElement<T>;

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T, const D: usize> Default for BBox<T, D>
where
    T: MatrixElement<T>,
{
    fn default() -> Self {
        Self([(T::default(), T::default()); D])
    }
}

impl<T, const D: usize> BBox<T, D>
where
    T: MatrixElement<T>,
{
    /// Return a new BoundingBox given 2 points at oposite corners.
    ///
    pub fn new(p1: Point<T, D>, p2: Point<T, D>) -> BBox<T, D>
    where
        T: MatrixElement<T>,
    {
        let mut bbox = BBox::default();

        bbox.0
            .iter_mut()
            .zip(p1.into_iter().zip(p2.into_iter()))
            .for_each(|((min, max), (c1, c2))| {
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
pub trait Bounding<T, const D: usize> {
    fn bounds(&self, point: &Point<T, D>) -> bool
    where
        T: MatrixElement<T>;
}

impl<T, const D: usize> Bounding<T, D> for BBox<T, D>
where
    T: MatrixElement<T>,
{
    /// Return true if a point lies within a bounding box. Return else otherwise
    ///
    fn bounds(&self, point: &Point<T, D>) -> bool
    where
        T: MatrixElement<T>,
    {
        point
            .into_iter()
            .zip(self.0.iter())
            .all(|(coord, &(min, max))| (min..=max).contains(&coord))
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
            Point::new([60.0, 100.0, -20.0, 0.0]),
        );

        let point_bound = Point::new([32.6, 50.29, -50.3, -0.1]);
        let point_not_bound = Point::new([32.6, 0.0, -50.3, -0.1]);

        assert!(bbox.bounds(&point_bound));
        assert!(!bbox.bounds(&point_not_bound));
    }
}
