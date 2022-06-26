//! Implementations of iterator and iterator operations on vectors.
//! 

use super::Vector;

impl<'a, const D: usize> IntoIterator for &'a Vector<D> {
    type Item = f64;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[f64; D], 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const D: usize> Vector<D> {
    /// Return an iterator over a mutable slice, containing a vector's components.
    ///
    pub fn iter(&mut self) -> std::iter::Flatten<std::slice::Iter<'_, [f64; D]>> {
        self.0.iter()
    }

    /// Returns an iterator over a vector's coordinates that allows modifying each value.
    ///
    pub fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [f64; D]>> {
        self.0.iter_mut()
    }

    /// Return a new vector where each coordinate has been modified acording to the closure f.
    ///
    pub fn map<F>(&self, f: F) -> Vector<D>
    where
        F: Fn(f64) -> f64,
    {
        Vector(self.0.map(f))
    }

    /// Apply the closure f to each of a vector's coordinates.
    ///
    #[allow(dead_code)]
    pub fn for_each_coord<F>(&mut self, f: F)
    where
        F: FnMut(&mut f64),
    {
        self.iter_mut().for_each(f);
    }
}