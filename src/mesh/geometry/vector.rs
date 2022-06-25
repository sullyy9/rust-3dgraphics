//! Implementation of Vector types.
//!

use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use super::{dimension::Dim, matrix::Matrix, point::Point};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type representing a N dimensional vector.
///
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector<const D: usize>(pub Matrix<1, D>);

////////////////////////////////////////////////////////////////////////////////
// Constructor Implementations /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Default for Vector<D> {
    fn default() -> Self {
        Self(Matrix::new([[0.0; D]]))
    }
}

impl<const D: usize> Vector<D> {
    /// Return a new Vector, given it's x, y and z components.
    ///
    pub fn new<T>(components: [T; D]) -> Self
    where
        T: Into<f64>,
    {
        Self(Matrix::new([components.map(|comp| comp.into())]))
    }

    /// Return a new Vector giving the magnitude and direction of one point to another.
    ///
    pub fn from_points(tail: Point<D>, head: Point<D>) -> Vector<D> {
        head.sub(tail)
    }

    /// Promote a vector to a higher dimentional vector where the additional dimensions are
    /// initialised as 0.
    ///
    pub fn promote<const ND: usize>(&self) -> Vector<ND> {
        let mut new_vector = Vector::default();
        let len = self.0[0].len();
        new_vector.0[0][..len].clone_from_slice(&self.0[0]);
        new_vector
    }

    /// Demote a vector to a lower dimentional vector.
    ///
    pub fn demote<const ND: usize>(&self) -> Vector<ND> {
        let mut new_vector = Vector::default();
        let len = new_vector.0[0].len();
        new_vector.0[0].clone_from_slice(&self.0[0][..len]);
        new_vector
    }
}

////////////////////////////////////////////////////////////////////////////////
// Method Implementations //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Vector<D> {
    /// Return a new Vector3D object, normal to the 2 given vectors.
    ///
    pub fn normal_to(vector1: Vector<D>, vector2: Vector<D>) -> Vector<D> {
        // Calculate the cross product of the 2 given vectors to get a vector perpendicular to
        // both.
        let mut normal_vector: Vector<D> = Vector::default();
        normal_vector.0[0][0] =
            (vector1.0[0][1] * vector2.0[0][2]) - (vector1.0[0][2] * vector2.0[0][1]);
        normal_vector.0[0][1] =
            (vector1.0[0][2] * vector2.0[0][0]) - (vector1.0[0][0] * vector2.0[0][2]);
        normal_vector.0[0][2] =
            (vector1.0[0][0] * vector2.0[0][1]) - (vector1.0[0][1] * vector2.0[0][0]);

        // Normalise the vector (It's magnitude should be 1).
        normal_vector /= f64::sqrt(
            normal_vector.0[0][0].powi(2)
                + normal_vector.0[0][1].powi(2)
                + normal_vector.0[0][2].powi(2),
        );
        normal_vector
    }

    /// Return the magnitude of the vector.
    ///
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.into_iter().fold(0.0, |sum, coord| sum + coord.powi(2)))
    }

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
    fn map<F>(&self, f: F) -> Vector<D>
    where
        F: Fn(f64) -> f64,
    {
        Vector(self.0.map(f))
    }

    /// Apply the closure f to each of a vector's coordinates.
    ///
    #[allow(dead_code)]
    fn for_each_coord<F>(&mut self, f: F)
    where
        F: FnMut(&mut f64),
    {
        self.iter_mut().for_each(f);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait Implementations ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<'a, const D: usize> IntoIterator for &'a Vector<D> {
    type Item = f64;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<[f64; D], 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Operator Overloads //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl<const D: usize> Index<Dim> for Vector<D> {
    type Output = f64;

    fn index(&self, index: Dim) -> &Self::Output {
        match index {
            Dim::X if D >= 1 => &self.0[0][0],
            Dim::Y if D >= 2 => &self.0[0][1],
            Dim::Z if D >= 3 => &self.0[0][2],
            Dim::W if D >= 4 => &self.0[0][3],
            Dim::N(n) if D >= n => &self.0[0][n],
            _ => panic!(),
        }
    }
}
impl<const D: usize> IndexMut<Dim> for Vector<D> {
    fn index_mut(&mut self, index: Dim) -> &mut Self::Output {
        match index {
            Dim::X if D >= 1 => &mut self.0[0][0],
            Dim::Y if D >= 2 => &mut self.0[0][1],
            Dim::Z if D >= 3 => &mut self.0[0][2],
            Dim::W if D >= 4 => &mut self.0[0][3],
            Dim::N(n) if D >= n => &mut self.0[0][n],
            _ => panic!(),
        }
    }
}

/// Vector + Vector = Vector
///
impl<const D: usize> Add<Vector<D>> for Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<const D: usize> Add<&Vector<D>> for Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: &Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<const D: usize> Add<Vector<D>> for &Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}
impl<const D: usize> Add<&Vector<D>> for &Vector<D> {
    type Output = Vector<D>;
    fn add(self, rhs: &Vector<D>) -> Self::Output {
        Vector(self.0.add(rhs.0))
    }
}

/// Vector += Vector
///
impl<const D: usize> AddAssign<Vector<D>> for Vector<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<const D: usize> AddAssign<&mut Vector<D>> for Vector<D> {
    fn add_assign(&mut self, rhs: &mut Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<const D: usize> AddAssign<Vector<D>> for &mut Vector<D> {
    fn add_assign(&mut self, rhs: Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}
impl<const D: usize> AddAssign<&mut Vector<D>> for &mut Vector<D> {
    fn add_assign(&mut self, rhs: &mut Vector<D>) {
        self.0.add_assign(rhs.0);
    }
}

/// Scaler Arithmetic.
///
/// Vector * Scaler = Vector.
///
impl<T: Into<f64>, const D: usize> Mul<T> for Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}
impl<T: Into<f64>, const D: usize> Mul<T> for &Vector<D> {
    type Output = Vector<D>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector(self.0.mul(rhs))
    }
}

/// Vector *= Scaler.
///
impl<T: Into<f64>, const D: usize> MulAssign<T> for Vector<D> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
    }
}
impl<T: Into<f64>, const D: usize> MulAssign<T> for &mut Vector<D> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.mul_assign(rhs);
    }
}

/// Vector / Scaler = Vector.
///
impl<T: Into<f64>, const D: usize> Div<T> for Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: T) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}
impl<T: Into<f64>, const D: usize> Div<T> for &Vector<D> {
    type Output = Vector<D>;
    fn div(self, rhs: T) -> Self::Output {
        Vector(self.0.div(rhs))
    }
}

/// Vector /= Scaler.
///
impl<T: Into<f64>, const D: usize> DivAssign<T> for Vector<D> {
    fn div_assign(&mut self, rhs: T) {
        self.0.div_assign(rhs);
    }
}
impl<T: Into<f64>, const D: usize> DivAssign<T> for &mut Vector<D> {
    fn div_assign(&mut self, rhs: T) {
        self.0.div_assign(rhs);
    }
}

/// -Vector = Vector
///
impl<const D: usize> Neg for Vector<D> {
    type Output = Vector<D>;
    fn neg(self) -> Self::Output {
        self.map(|coord| coord.neg())
    }
}
impl<const D: usize> Neg for &Vector<D> {
    type Output = Vector<D>;
    fn neg(self) -> Self::Output {
        self.map(|coord| coord.neg())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests ///////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaler_mul() {
        let coords = [0.43, 56.28, -87.52, -0.23];
        let scaler = 4.87;

        let coords_scaled = coords.map(|coord| coord * 4.87);

        assert_eq!(Vector::new(coords) * scaler, Vector::new(coords_scaled));

        let mut point_mul_assign = Vector::new(coords);
        point_mul_assign *= scaler;
        assert_eq!(point_mul_assign, Vector::new(coords_scaled));
    }

    #[test]
    fn test_scaler_div() {
        let coords = [0.43, 56.28, -87.52, -0.23];
        let scaler = 4.87;

        let coords_scaled = coords.map(|coord| coord / 4.87);

        assert_eq!(Vector::new(coords) / scaler, Vector::new(coords_scaled));

        let mut point_mul_assign = Vector::new(coords);
        point_mul_assign /= scaler;
        assert_eq!(point_mul_assign, Vector::new(coords_scaled));
    }
}
