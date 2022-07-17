
use super::{Point, Vector, Polygonal};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    None,
    Partial,
    Full,
}

pub struct Polygon<'a> {
    pub vertex: [&'a Point<4>; 3],
    pub normal: Option<&'a Vector<3>>,
    pub visible: Option<&'a Visibility>,
}

impl Polygonal for Polygon<'_> {
    fn vertex_count(&self) -> usize {
        3
    }

    fn verticies(&self) -> &[&Point<4>] {
        &self.vertex[..]
    }

    fn normal(&self) -> Vector<3> {
        todo!()
    }
}