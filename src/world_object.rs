use crate::{physics::PhysicalState, mesh::Pipeline};

////////////////////////////////////////////////////////////////////////////////
///Types & Traits //////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub struct WorldObject<T> {
    mesh: T,

    pub physics: PhysicalState,
}

impl<T> WorldObject<T> {
    
}