use super::{geometry::Vector3D, static_mesh::StaticMesh, vertex::Vertex};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type used for storage of a mesh that can then be transformed. This type also contains aditional information
/// such as polygon normals, polygon visibility, etc.
///
pub struct DynamicMesh {
    vertex_components: Vec<Vertex>, // List of verticies.

    polygon_components: Vec<[usize; 3]>, // List of indeicies to the verticies that compose a polygon.
    normal_components: Vec<Vector3D>,    // List of normal components of a polygon.
    visibility_index: Vec<usize>, // List of indicies to polygon and normal components that compose visible polygons.
}

/// Iterator type for iterating over a DynamicMesh's polygons.
///
pub struct PolyIterator<'a> {
    vertex_components: &'a [Vertex],

    polygon_components: &'a [[usize; 3]],
    normal_components: &'a [Vector3D],
}

pub struct Polygon<'a> {
    pub verticies: [&'a Vertex; 3],
    pub normal: &'a Vector3D,
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for DynamicMesh {
    fn default() -> Self {
        Self {
            vertex_components: Default::default(),
            polygon_components: Default::default(),
            normal_components: Default::default(),
            visibility_index: Default::default(),
        }
    }
}

impl DynamicMesh {
    pub fn new(mesh: &StaticMesh) -> Self {
        let number_polygons = mesh.polygon_components.len();
        Self {
            vertex_components: mesh.vertex_components.to_vec(),
            polygon_components: mesh.polygon_components.to_vec(),
            normal_components: Vec::with_capacity(number_polygons),
            visibility_index: Vec::new(),
        }
    }
}

impl<'a> Iterator for PolyIterator<'a> {
    type Item = Polygon<'a>;

    ///
    /// Get the next item.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.polygon_components.is_empty() {
            None
        } else {
            let ref_polygon = {
                // Split off a reference to the first polygon in the slice and give the reference to the rest of the
                // list back to the iterator struct.
                let (polygon_component, remaining_list) = self.polygon_components.split_first()?;
                self.polygon_components = remaining_list;

                let (normal_component, remaining_list) = self.normal_components.split_first()?;
                self.normal_components = remaining_list;

                // Construct a polygon of references from the index polygon and vertex list.
                Polygon {
                    verticies: [
                        &self.vertex_components[polygon_component[0]],
                        &self.vertex_components[polygon_component[1]],
                        &self.vertex_components[polygon_component[2]],
                    ],
                    normal: normal_component,
                }
            };

            Some(ref_polygon)
        }
    }
}
