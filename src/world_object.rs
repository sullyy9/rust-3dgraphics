use crate::{
    geometry::{Orientation, Point, Degrees},
    mesh::Renderable,
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
    pub orientation: Orientation<Degrees, 3>,
}

impl<T> WorldObject<T>
where
    T: Renderable,
{
    pub fn new(mesh: T) -> Self {
        WorldObject {
            mesh,
            position: Point::default(),
            orientation: Orientation::default(),
        }
    }
}
