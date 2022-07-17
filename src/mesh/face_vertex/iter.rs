//!
//!

use super::{PipeMesh, Point, Polygon, VIndex, Vector, Visibility};

pub struct Iter<'a> {
    vertex: &'a [Point<4>],
    vindex: &'a [VIndex],
    normal: Option<&'a [Vector<3>]>,
    visible: Option<&'a [Visibility]>,
}

impl<'a> Iter<'a> {
    pub fn new<T>(mesh: &'a PipeMesh<'a, T>) -> Iter<'a> {
        Iter {
            vertex: &mesh.vertex,
            vindex: &mesh.vindex,
            normal: mesh.normal.as_deref(),
            visible: mesh.visible.as_deref(),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Polygon<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vindex.is_empty() {
            None
        } else {
            let normal = if let Some(normals) = self.normal {
                let (first, remainder) = normals.split_first()?;
                self.normal = Some(remainder);
                Some(first)
            } else {
                None
            };

            let visible = if let Some(visibility) = self.visible {
                let (first, remainder) = visibility.split_first()?;
                self.visible = Some(remainder);
                Some(first)
            } else {
                None
            };

            let (vindex, remaining) = self.vindex.split_first()?;
            self.vindex = remaining;

            Some(Polygon {
                vertex: [
                    &self.vertex[vindex.0[0]],
                    &self.vertex[vindex.0[1]],
                    &self.vertex[vindex.0[2]],
                ],
                normal,
                visible,
            })
        }
    }

    // fn for_each<F>(self, mut f: F)
    // where
    //     Self: Sized,
    //     F: FnMut(Self::Item),
    // {
    //     for polygon in self {
    //         f(polygon);
    //     }
    // }
}
