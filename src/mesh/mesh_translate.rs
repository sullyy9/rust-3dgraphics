//! Implementation of translation and rotation methods for the mesh structure.
//!

#[allow(dead_code)]
impl Mesh {
    ///
    /// Set the mesh's absolute orientation.
    ///
    pub fn abs_orientation(&mut self, x: f32, y: f32, z: f32) {
        self.orientation.x = x.clamp(-180.0, 180.0);
        self.orientation.y = y.clamp(-180.0, 180.0);
        self.orientation.z = z.clamp(-180.0, 180.0);
    }

    ///
    /// Set the mesh's absolute orientation.
    ///
    pub fn rel_orientation(&mut self, x: f32, y: f32, z: f32) {
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

    ///
    /// Set the mesh's absolute position.
    ///
    pub fn abs_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    ///
    /// Set the mesh's position relative to it's current position.
    ///
    pub fn rel_position(&mut self, x: f32, y: f32, z: f32) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }
}
