//! Implementation of a structure containg the kinematic states that represent an objects physical location.
//!

use crate::mesh::geometry::{Orientation3D, Point3D};

///
/// Representation of an objects kinematic state within a cartesian coordinate system.
///
#[derive(Clone)]
pub struct PhysicalState {
    pub position: Point3D,
    pub orientation: Orientation3D,
}

impl Default for PhysicalState {
    /// Create a new physical state with position initialised at the origin and no rotation.
    ///
    fn default() -> Self {
        Self {
            position: Default::default(),
            orientation: Default::default(),
        }
    }
}

impl PhysicalState {
    ///
    /// Create a new physical state with position initialised at the origin and no rotation.
    ///
    pub fn new() -> PhysicalState {
        PhysicalState {
            position: Point3D::new([0, 0, 0]),
            orientation: Orientation3D::new(0, 0, 0),
        }
    }
}
