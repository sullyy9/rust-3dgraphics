//! Implementations of iterator and iterator operations on vectors.
//!

use super::{MatrixElement, Vector};

impl<'a, T, const D: usize> IntoIterator for &'a Vector<T, D>
where
    T: MatrixElement<T>,
{
    type Item = T;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[T; D], 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const D: usize> Vector<T, D>
where
    T: MatrixElement<T>,
{
    /// Return an iterator over a mutable slice, containing a vector's components.
    ///
    pub fn iter(&mut self) -> std::iter::Flatten<std::slice::Iter<'_, [T; D]>> {
        self.0.iter()
    }

    /// Returns an iterator over a vector's coordinates that allows modifying each value.
    ///
    pub fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [T; D]>> {
        self.0.iter_mut()
    }

    /// Return a new vector where each coordinate has been modified acording to the closure f.
    ///
    pub fn map<U, F>(&self, f: F) -> Vector<U, D>
    where
        U: MatrixElement<U>,
        F: FnMut(T) -> U,
    {
        Vector(self.0.map(f))
    }

    /// Apply the closure f to each of a vector's coordinates.
    ///
    #[allow(dead_code)]
    pub fn for_each_coord<F>(&mut self, f: F)
    where
        F: FnMut(&mut T),
    {
        self.iter_mut().for_each(f);
    }
}
