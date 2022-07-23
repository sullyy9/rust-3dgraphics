use crate::{
    geometry::{Degrees, Orientation, Point, RotationAxis, Vector},
    mesh::Transform,
};

pub struct Camera {
    pub position: Point<3>,
    pub orientation: Orientation<Degrees, 3>,
}

impl Camera {
    pub fn new(position: Point<3>) -> Self {
        Camera {
            position,
            orientation: Orientation::new([Degrees(0.0), Degrees(0.0), Degrees(0.0)]),
        }
    }

    pub fn view_transform(&self) -> Transform {
        Transform::builder()
            .translate(self.position.vector_to(&Point::new([0, 0, 0])))
            .rotate_about_x(-self.orientation[RotationAxis::Roll])
            .rotate_about_y(-self.orientation[RotationAxis::Pitch])
            .rotate_about_z(-self.orientation[RotationAxis::Yaw])
            .build_affine()
    }

    pub fn move_relative(&mut self, movement: Vector<3>) {
        let movement: Vector<3> = {
            let transform = Transform::builder()
            .translate(self.position.vector_from(&Point::new([0, 0, 0])))
            .rotate_about_x(self.orientation[RotationAxis::Roll])
            .rotate_about_y(self.orientation[RotationAxis::Pitch])
            .rotate_about_z(self.orientation[RotationAxis::Yaw])
            .build_affine();

            let mut homogenous: Vector<4> = movement.promote();
            homogenous *= transform;
            homogenous.demote()
        };

        self.position += movement;
    }
}
