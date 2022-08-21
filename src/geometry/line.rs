use super::{Dim, MatrixElement, Point};

pub struct LineSegment<T, const D: usize>(pub Point<T, D>, pub Point<T, D>)
where
    T: MatrixElement<T>;

impl<T, const D: usize> LineSegment<T, D>
where
    T: MatrixElement<T>,
{
    pub fn new(p1: Point<T, D>, p2: Point<T, D>) -> Self {
        Self(p1, p2)
    }
}

impl<T, const D: usize> LineSegment<T, D>
where
    T: MatrixElement<T> + Into<f64>,
{
    /// Find the gradient of the lines 2D projection.
    /// 
    pub fn gradient_xy(&self) -> f64 {
        if self.0[Dim::X] <= self.1[Dim::X] {
            let dx = (self.1[Dim::X] - self.0[Dim::X]).into();
            let dy = (self.1[Dim::Y] - self.0[Dim::Y]).into();
            dy / dx
        } else {
            let dx = (self.0[Dim::X] - self.1[Dim::X]).into();
            let dy = (self.0[Dim::Y] - self.1[Dim::Y]).into();
            dy / dx
        }
    }
}
