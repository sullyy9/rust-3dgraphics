use super::{geometry::Vector, Vertex};

///
/// Polygon
///
#[derive(Copy, Clone)]
pub struct Polygon<T, U> {
    pub verticies: [T; 3],

    pub normal: U,
}
impl<T, U> Polygon<T, U> {
    pub fn new(p1: T, p2: T, p3: T, normal: U) -> Polygon<T, U> {
        Polygon {
            verticies: [p1, p2, p3],
            normal,
        }
    }
}

/// Polygon where verticies are indexes to lists.
pub type IndexPoly = Polygon<usize, usize>;

/// Polygons where all members are references.
pub type RefPoly<'a> = Polygon<&'a Vertex, &'a Vector<3>>;
