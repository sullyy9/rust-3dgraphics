//! Implementations of iterator and iterator operations on points.
//!

use super::{MatrixElement, Point};

impl<'a, T, const D: usize> IntoIterator for &'a Point<T, D>
where
    T: MatrixElement<T>,
{
    type Item = T;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[T; D], 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: MatrixElement<T>,
{
    /// Return an iterator over a mutable slice, containing a point's coordinates.
    ///
    pub fn iter(&mut self) -> std::iter::Flatten<std::slice::Iter<'_, [T; D]>> {
        self.0.iter()
    }

    /// Return an iterator over a mutable slice, containing a point's coordinates.
    ///
    pub fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [T; D]>> {
        self.0.iter_mut()
    }

    /// Return a new point where each coordinate has been modified acording to the closure f.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    #[allow(dead_code)]
    pub fn map<F>(&self, f: F) -> Point<T, D>
    where
        F: Fn(T) -> T,
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
        F: Fn(&mut T),
    {
        self.0.for_each(f);
    }
}
