use crate::mesh::{geometry::Orientation3D, Point, Transform};

#[derive(Default)]
pub struct Camera {
    pub position: Point<3>,
    pub orientation: Orientation3D,
}

impl Camera {
    pub fn new(position: Point<3>) -> Self {
        Camera {
            position,
            orientation: Orientation3D::default(),
        }
    }

    pub fn view_transform(&self) -> Transform {
        Transform::builder()
            .translate(self.position.vector_to(&Point::new([0, 0, 0])))
            .rotate_about_x(-self.orientation.x)
            .rotate_about_y(-self.orientation.y)
            .rotate_about_z(-self.orientation.z)
            .build_affine()
    }
}
