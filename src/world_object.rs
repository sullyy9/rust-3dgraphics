use crate::{
    mesh::{geometry::Orientation3D, Point, Renderable},
};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub struct WorldObject<T>
where
    T: Renderable,
{
    pub mesh: T,
    pub position: Point<3>,
    pub orientation: Orientation3D,
}

impl<T> WorldObject<T>
where
    T: Renderable,
{
    pub fn new(mesh: T) -> Self {
        WorldObject {
            mesh,
            position: Point::default(),
            orientation: Orientation3D::default(),
        }
    }
}
