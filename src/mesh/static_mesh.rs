use super::vertex::Vertex;

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// Type used for static storage of an object's a mesh.
/// 
pub struct StaticMesh {
    pub vertex_components: Vec<Vertex>,
    pub polygon_components: Vec<[usize; 3]>,
}

////////////////////////////////////////////////////////////////////////////////
/// Implementations ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

impl Default for StaticMesh {
    fn default() -> Self {
        Self {
            vertex_components: Default::default(),
            polygon_components: Default::default(),
        }
    }
}

impl StaticMesh {
    /// Load a cube into the mesh.
    ///
    pub fn load_cube<T: Into<f64>>(&mut self, edge_length: T) {
        let edge_length = edge_length.into();
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        self.vertex_components.push(Vertex::new(neg, neg, pos, 1));
        self.vertex_components.push(Vertex::new(pos, neg, pos, 1));
        self.vertex_components.push(Vertex::new(neg, neg, neg, 1));
        self.vertex_components.push(Vertex::new(pos, neg, neg, 1));
        self.vertex_components.push(Vertex::new(neg, pos, pos, 1));
        self.vertex_components.push(Vertex::new(pos, pos, pos, 1));
        self.vertex_components.push(Vertex::new(neg, pos, neg, 1));
        self.vertex_components.push(Vertex::new(pos, pos, neg, 1));

        self.polygon_components.push([2, 6, 7]);
        self.polygon_components.push([2, 7, 3]);
        self.polygon_components.push([3, 7, 5]);
        self.polygon_components.push([3, 5, 1]);
        self.polygon_components.push([1, 5, 4]);
        self.polygon_components.push([1, 4, 0]);
        self.polygon_components.push([0, 4, 6]);
        self.polygon_components.push([0, 6, 2]);
        self.polygon_components.push([6, 4, 5]);
        self.polygon_components.push([6, 5, 7]);
        self.polygon_components.push([0, 2, 3]);
        self.polygon_components.push([0, 3, 1]);

    }
}
