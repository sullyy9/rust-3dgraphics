//! Implementation of a structure containg the kinematic states that represent an objects physical location.
//!

///
/// 3D point in a cartesian coordinate system.
///
#[derive(Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

///
/// 3D rotation using Euler angles in a cartesian coordinate system.
///
#[derive(Clone)]
pub struct Rotation3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

///
/// Representation of an objects kinematic state within a cartesian coordinate system.
///
#[derive(Clone)]
pub struct PhysicalState {
    pub position: Point3D,
    pub orientation: Rotation3D,
}
impl PhysicalState {
    ///
    /// Create a new physical state with position initialised at the origin and no rotation.
    ///
    pub fn new() -> PhysicalState {
        PhysicalState {
            position: Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            orientation: Rotation3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

pub trait PhysicalObject {
    fn set_absolute_position(&mut self, x: f64, y: f64, z: f64);
    fn set_relative_position(&mut self, x: f64, y: f64, z: f64);
    fn set_absolute_orientation(&mut self, x: f64, y: f64, z: f64);
    fn set_relative_orientation(&mut self, x: f64, y: f64, z: f64);
}

impl PhysicalObject for PhysicalState {
    ///
    /// Set the absolute position.
    ///
    fn set_absolute_position(&mut self, x: f64, y: f64, z: f64) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    ///
    /// Set the position relative to the current position.
    ///
    fn set_relative_position(&mut self, x: f64, y: f64, z: f64) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }

    ///
    /// Set the absolute orientation.
    ///
    fn set_absolute_orientation(&mut self, x: f64, y: f64, z: f64) {
        self.orientation.x = x.clamp(-180.0, 180.0);
        self.orientation.y = y.clamp(-180.0, 180.0);
        self.orientation.z = z.clamp(-180.0, 180.0);
    }

    ///
    /// Set the mesh's absolute orientation.
    ///
    fn set_relative_orientation(&mut self, x: f64, y: f64, z: f64) {
        self.orientation.x += x;
        self.orientation.y += y;
        self.orientation.z += z;

        while self.orientation.x > 180.0 {
            self.orientation.x -= 360.0;
        }
        while self.orientation.x < 180.0 {
            self.orientation.x += 360.0;
        }

        while self.orientation.y > 180.0 {
            self.orientation.y -= 360.0;
        }
        while self.orientation.y < 180.0 {
            self.orientation.y += 360.0;
        }

        while self.orientation.y > 180.0 {
            self.orientation.y -= 360.0;
        }
        while self.orientation.y < 180.0 {
            self.orientation.y += 360.0;
        }
    }
}
