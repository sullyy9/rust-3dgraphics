use std::ops::{Index, IndexMut};

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing an N dimensional Matrix.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize>(pub(super) [[f64; C]; R]);

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
    /// Construct a matrix where each element is set to 0.
    ///
    fn default() -> Self {
        Self([[0.0; C]; R])
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(data: [[f64; C]; R]) -> Self {
        Matrix(data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Method Implementations //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const R: usize, const C: usize> Matrix<R, C> {
    /// Return a new matrix where each element has been modified acording to the closure f.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    pub(super) fn map<F>(&self, f: F) -> Matrix<R, C>
    where
        F: Fn(f64) -> f64,
    {
        Matrix::new(self.0.map(|row| row.map(&f)))
    }

    /// Apply the closure f to each of a point's coordinates.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    pub(super) fn for_each<F>(&mut self, f: F)
    where
        F: Fn(&mut f64),
    {
        self.0
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(&f));
    }

    /// Iterate over each element starting 0,0 then 0,1, 0,2, etc.
    ///
    pub(super) fn iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, [f64; C]>>
    {
        self.0.iter().flatten()
    }

    /// Iterate over each element starting 0,0 then 0,1, 0,2, etc.
    ///
    pub(super) fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [f64; C]>>
    {
        self.0.iter_mut().flatten()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const R: usize, const C: usize> Index<usize> for Matrix<R, C> {
    type Output = [f64; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for Matrix<R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let mat = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]);

        assert_eq!(mat[0][1], 1.0);
        assert_eq!(mat[1][3], 7.0);
    }

    #[test]
    #[should_panic]
    fn test_index_panic() {
        let mat = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]);

        let _should_panic = mat[2][1];
    }

    #[test]
    fn test_scalar_mul() {
        let mut mat = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]) * 4;
        assert_eq!(mat[1][1], 20.0);

        mat *= 2;
        assert_eq!(mat[1][1], 40.0);
    }

    #[test]
    fn test_add() {
        let mut mat1 = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]);
        let mat2 = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]);
        assert_eq!((mat1 + mat2)[1][1], 10.0);

        mat1 += mat2;
        assert_eq!(mat1[1][1], 10.0)
    }

    #[test]
    fn test_sub() {
        let mut mat1 = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 7.0, 6.0, 7.0]]);
        let mat2 = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]);
        assert_eq!((mat1 - mat2)[1][1], 2.0);

        mat1 += mat2;
        assert_eq!(mat1[1][1], 2.0);
    }
}
