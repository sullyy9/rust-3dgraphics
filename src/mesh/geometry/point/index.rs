//! Implementations of indexing for points.
//! 

use std::ops::{Index, IndexMut};

use super::{Point, Dim};

impl<const D: usize> Index<Dim> for Point<D> {
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
impl<const D: usize> IndexMut<Dim> for Point<D> {
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