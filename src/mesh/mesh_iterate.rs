//! Implementations of methods for iterating over a mesh's components. 
//! 

pub struct PolyIterator<'a> {
    vertex_list: &'a [prim::Vertex],
    normal_list: &'a [prim::Vector],
    polygon_list: &'a [prim::IndexPoly],
}
#[allow(dead_code)]
impl Mesh {
    ///
    /// Iterate over all polygons immutably.
    ///
    pub fn iter_all_polygons<'a>(&'a self) -> PolyIterator<'a> {
        let vertex_list = self.verticies.as_slice();
        let normal_list = self.normals.as_slice();
        let polygon_list = self.polygons.as_slice();

        PolyIterator {
            vertex_list,
            normal_list,
            polygon_list,
        }
    }

    ///
    /// Iterate over only visible polygons immutably.
    ///
    pub fn iter_visible_polygons<'a>(&'a self) -> PolyIterator<'a> {
        let vertex_list = self.verticies.as_slice();
        let normal_list = self.normals.as_slice();
        let polygon_list = self.visible_polygons.as_slice();

        PolyIterator {
            vertex_list,
            normal_list,
            polygon_list,
        }
    }
}
impl<'a> Iterator for PolyIterator<'a> {
    type Item = prim::RefPoly<'a>;

    ///
    /// Get the next item.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.polygon_list.is_empty() {
            None
        } else {
            let ref_polygon = {
                // Split off a reference to the first polygon in the slice and give the reference to the rest of the
                // list back to the iterator struct.
                let (index_poly, remaining_list) = self.polygon_list.split_first()?;
                self.polygon_list = remaining_list;

                // Construct a polygon of references from the index polygon and vertex list.
                prim::RefPoly::new(
                    &self.vertex_list[index_poly.p1],
                    &self.vertex_list[index_poly.p2],
                    &self.vertex_list[index_poly.p3],
                    &self.normal_list[index_poly.normal],
                )
            };

            Some(ref_polygon)
        }
    }
}