//! Implementations of a face-vertex mesh data structure and methods construction methods.
//! 

/// The mesh consists of a number of verticies and polygons. Each polygon contains 3 indicies
/// to the positions of its verticies in the verticies vector.
#[derive(Clone)]
pub struct Mesh {
    verticies: Vec<prim::Vertex>,
    normals: Vec<prim::Vector>,
    polygons: Vec<prim::IndexPoly>,

    visible_polygons: Vec<prim::IndexPoly>,

    pub position: prim::Vertex,
    pub orientation: prim::Vertex,
}
impl Mesh {
    ///
    /// Create an empty mesh
    ///
    pub fn new() -> Mesh {
        let verticies = Vec::new();
        let normals = Vec::new();
        let polygons = Vec::new();
        let visible_polygons = Vec::new();

        let position = prim::Vertex::new(0.0, 0.0, 0.0, 0.0);
        let orientation = prim::Vertex::new(0.0, 0.0, 0.0, 0.0);

        Mesh {
            verticies,
            normals,
            polygons,
            visible_polygons,
            position,
            orientation,
        }
    }

    ///
    /// Load a cube into the mesh
    ///
    pub fn load_cube(&mut self, edge_length: f32) {
        let pos = edge_length / 2.0;
        let neg = -edge_length / 2.0;

        self.verticies.push(prim::Vertex::new(neg, neg, pos, 1.0));
        self.verticies.push(prim::Vertex::new(pos, neg, pos, 1.0));
        self.verticies.push(prim::Vertex::new(neg, neg, neg, 1.0));
        self.verticies.push(prim::Vertex::new(pos, neg, neg, 1.0));
        self.verticies.push(prim::Vertex::new(neg, pos, pos, 1.0));
        self.verticies.push(prim::Vertex::new(pos, pos, pos, 1.0));
        self.verticies.push(prim::Vertex::new(neg, pos, neg, 1.0));
        self.verticies.push(prim::Vertex::new(pos, pos, neg, 1.0));

        self.polygons.push(prim::IndexPoly::new(2, 6, 7, 0));
        self.polygons.push(prim::IndexPoly::new(2, 7, 3, 1));
        self.polygons.push(prim::IndexPoly::new(3, 7, 5, 2));
        self.polygons.push(prim::IndexPoly::new(3, 5, 1, 3));
        self.polygons.push(prim::IndexPoly::new(1, 5, 4, 4));
        self.polygons.push(prim::IndexPoly::new(1, 4, 0, 5));
        self.polygons.push(prim::IndexPoly::new(0, 4, 6, 6));
        self.polygons.push(prim::IndexPoly::new(0, 6, 2, 7));
        self.polygons.push(prim::IndexPoly::new(6, 4, 5, 8));
        self.polygons.push(prim::IndexPoly::new(6, 5, 7, 9));
        self.polygons.push(prim::IndexPoly::new(0, 2, 3, 10));
        self.polygons.push(prim::IndexPoly::new(0, 3, 1, 11));

        for _ in self.polygons.iter() {
            self.normals.push(prim::Vector::new(0.0, 0.0, 0.0, 0.0));
        }
    }
}