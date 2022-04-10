//! Implementation of primitive geometric data types. E.g. 3D point, 3D vector, etc.
//!

pub mod vertex;

///
/// Polygon
///
#[derive(Copy, Clone)]
pub struct Polygon<T> {
    pub p1: T,
    pub p2: T,
    pub p3: T,

    pub normal: T,
}
impl<T> Polygon<T> {
    pub fn new(p1: T, p2: T, p3: T, normal: T) -> Polygon<T> {
        Polygon { p1, p2, p3, normal }
    }
}

/// Polygon where verticies are indexes to lists.
pub type IndexPoly = Polygon<usize>;

/// Polygons where all members are references.
pub type RefPoly<'a> = Polygon<&'a vertex::Vertex>;