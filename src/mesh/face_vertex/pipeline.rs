use super::{Bounding, Dim, PipeMesh, Scalar, Transform, Vector, Visibility};

pub trait Pipeline {
    fn transform(self, transform: &Transform) -> Self;
    fn update_normals(self) -> Self;
    fn update_visibility<B>(self, bounder: &B) -> Self
    where
        B: Bounding<f64, 4>;
}

impl Pipeline for PipeMesh {
    fn transform(mut self, transform: &Transform) -> Self {
        self.vertex.iter_mut().for_each(|mut vertex| {
            vertex *= transform;
            vertex /= Scalar(vertex[Dim::W]);
        });
        self
    }

    fn update_normals(mut self) -> Self {
        let normals: Vec<Vector<f64, 3>> = self
            .vindex
            .iter()
            .map(|i| {
                let vector1 = Vector::from_points(self.vertex[i.0[0]], self.vertex[i.0[1]]);
                let vector2 = Vector::from_points(self.vertex[i.0[0]], self.vertex[i.0[2]]);
                Vector::normal_to(vector1, vector2).demote()
            })
            .collect();

        self.normal = Some(normals.into_boxed_slice());
        self
    }

    fn update_visibility<B>(mut self, bounder: &B) -> Self
    where
        B: Bounding<f64, 4>,
    {
        let visible = self
            .vindex
            .iter()
            .map(|i| {
                if bounder.bounds(&self.vertex[i.0[0]])
                    && bounder.bounds(&self.vertex[i.0[1]])
                    && bounder.bounds(&self.vertex[i.0[2]])
                {
                    Visibility::Full
                } else if bounder.bounds(&self.vertex[i.0[0]])
                    || bounder.bounds(&self.vertex[i.0[1]])
                    || bounder.bounds(&self.vertex[i.0[2]])
                {
                    Visibility::Partial
                } else {
                    Visibility::None
                }
            })
            .collect();

        self.visible = Some(visible);
        self
    }
}
