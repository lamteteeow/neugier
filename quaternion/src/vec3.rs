use crate::quaternion::{Quaternion, UnitQuaternion};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn rotate(self, q: UnitQuaternion) -> Self {
        let r = Quaternion(0., self.0, self.1, self.2);
        let rx = q.inner() * r * q.inner().conjugate();
        Self(rx.1, rx.2, rx.3)
    }
}
