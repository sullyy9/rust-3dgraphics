//! Implementations of iterator and iterator operations on points.
//! 

use super::Point;

impl<'a, const D: usize> IntoIterator for &'a Point<D> {
    type Item = f64;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[f64; D], 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const D: usize> Point<D> {
    /// Return an iterator over a mutable slice, containing a point's coordinates.
    ///
    pub fn iter(&mut self) -> std::iter::Flatten<std::slice::Iter<'_, [f64; D]>> {
        self.0.iter()
    }

    /// Return an iterator over a mutable slice, containing a point's coordinates.
    ///
    pub fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [f64; D]>> {
        self.0.iter_mut()
    }

    /// Return a new point where each coordinate has been modified acording to the closure f.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    #[allow(dead_code)]
    pub fn map<F>(&self, f: F) -> Point<D>
    where
        F: Fn(f64) -> f64,
    {
        Point(self.0.map(f))
    }

    /// Apply the closure f to each of a point's coordinates.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    #[allow(dead_code)]
    pub fn for_each<F>(&mut self, f: F)
    where
        F: Fn(&mut f64),
    {
        self.0.for_each(f);
    }
}