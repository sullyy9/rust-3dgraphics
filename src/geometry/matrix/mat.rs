use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////
// Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing an N dimensional Matrix.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix<T, const R: usize, const C: usize>(pub(self) [[T; C]; R]);

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    /// Construct a matrix where each element is set to 0.
    ///
    fn default() -> Self {
        Self([[T::default(); C]; R])
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(data: [[T; C]; R]) -> Self {
        Self(data)
    }
}

pub trait MatrixElement<T>:
    Sized
    + Default
    + Copy
    + Add<T, Output = T>
    + AddAssign
    + Sub<T, Output = T>
    + SubAssign
    + Mul<T, Output = T>
    + MulAssign
    + Div<T, Output = T>
    + DivAssign
    + PartialOrd
    + PartialEq
{
}
impl MatrixElement<i8> for i8 {}
impl MatrixElement<i16> for i16 {}
impl MatrixElement<i32> for i32 {}
impl MatrixElement<i64> for i64 {}
impl MatrixElement<i128> for i128 {}
impl MatrixElement<isize> for isize {}
impl MatrixElement<u8> for u8 {}
impl MatrixElement<u16> for u16 {}
impl MatrixElement<u32> for u32 {}
impl MatrixElement<u64> for u64 {}
impl MatrixElement<u128> for u128 {}
impl MatrixElement<usize> for usize {}
impl MatrixElement<f32> for f32 {}
impl MatrixElement<f64> for f64 {}

////////////////////////////////////////////////////////////////////////////////
// Method Implementations //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: MatrixElement<T>,
{
    /// Return a new matrix where each element has been modified acording to the closure f.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    pub fn map<F>(&self, f: F) -> Matrix<T, R, C>
    where
        F: Fn(T) -> T,
    {
        Matrix(self.0.map(|row| row.map(&f)))
    }

    /// Apply the closure f to each of a point's coordinates.
    ///
    /// # Arguments
    /// * f - A closure which will be called on each coordinate.
    ///
    pub fn for_each<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        self.iter_mut().for_each(&f);
    }

    /// Iterate over each element starting 0,0 then 0,1, 0,2, etc.
    ///
    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, [T; C]>> {
        self.0.iter().flatten()
    }

    /// Iterate over each element starting 0,0 then 0,1, 0,2, etc.
    ///
    pub fn iter_mut(&mut self) -> std::iter::Flatten<std::slice::IterMut<'_, [T; C]>> {
        self.0.iter_mut().flatten()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T, const R: usize, const C: usize> IntoIterator for Matrix<T, R, C> {
    type Item = T;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[T; C], R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().flatten()
    }
}

impl<T, const R: usize, const C: usize> AsRef<Matrix<T, R, C>> for Matrix<T, R, C> {
    fn as_ref(&self) -> &Matrix<T, R, C> {
        self
    }
}
impl<T, const R: usize, const C: usize> AsMut<Matrix<T, R, C>> for Matrix<T, R, C> {
    fn as_mut(&mut self) -> &mut Matrix<T, R, C> {
        self
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(array: [[T; C]; R]) -> Self {
        Matrix(array.map(|row| row.map(|i| i)))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<T, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    type Output = [T; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
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
    use crate::geometry::Scalar;

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
        let mut mat = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]) * Scalar(4.0);
        assert_eq!(mat[1][1], 20.0);

        mat *= Scalar(2.0);
        assert_eq!(mat[1][1], 40.0);
    }

    #[test]
    fn test_scalar_div() {
        let mat = Matrix([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]) / Scalar(4.0);
        assert_eq!(mat[1][1], (5.0 / 4.0));
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

        mat1 -= mat2;
        assert_eq!(mat1[1][1], 2.0);
    }

    #[test]
    fn test_mul() {
        let mat1 = Matrix::from([[0, 1, 2], [3, 4, 5]]);
        let mat2 = Matrix::from([[6, 7], [8, 9], [10, 11]]);
        assert_eq!((mat1 * mat2), Matrix::from([[28, 31], [100, 112]]));

        let mut mat3 = Matrix::from([[2, 3], [5, 4]]);
        mat3 *= mat3;
        assert_eq!(mat3, Matrix::from([[19, 18], [30, 31]]));
    }
}
