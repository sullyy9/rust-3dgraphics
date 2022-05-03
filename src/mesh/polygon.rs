use super::{
    geometry::{BoundingBox, Point, Vector3D},
    Vertex,
};

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
pub type RefPoly<'a> = Polygon<&'a Vertex, &'a Vector3D>;

impl<'a> RefPoly<'a> {
    pub fn partially_bound_by(&self, bbox: &BoundingBox) -> bool {
        self.verticies[0].bound_by(bbox)
            || self.verticies[1].bound_by(bbox)
            || self.verticies[2].bound_by(bbox)
    }
}
