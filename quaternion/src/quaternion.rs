use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

#[derive(Debug, Clone, Copy)]
pub struct UnitQuaternion(Quaternion);

impl Quaternion {
    pub fn identity() -> Self {
        Self(1., 0., 0., 0.)
    }

    pub fn magnitude(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3).sqrt()
    }

    pub fn conjugate(self) -> Self {
        Self(self.0, -self.1, -self.2, -self.3)
    }

    pub fn normalize(self) -> Option<UnitQuaternion> {
        let mag = self.magnitude();
        if mag < f32::EPSILON {
            None
        } else {
            Some(UnitQuaternion(self / mag))
        }
    }

    pub fn inverse(self) -> Option<Self> {
        let mag = self.magnitude();
        if mag < f32::EPSILON {
            None
        } else {
            Some(self.conjugate() / (mag * mag))
        }
    }
}

impl Neg for Quaternion {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1 - self.2 * rhs.2 - self.3 * rhs.3,
            self.0 * rhs.1 + self.1 * rhs.0 + self.2 * rhs.3 - self.3 * rhs.2,
            self.0 * rhs.2 - self.1 * rhs.3 + self.2 * rhs.0 + self.3 * rhs.1,
            self.0 * rhs.3 + self.1 * rhs.2 - self.2 * rhs.1 + self.3 * rhs.0,
        )
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion(self * rhs.0, self * rhs.1, self * rhs.2, self * rhs.3)
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Div<f32> for Quaternion {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl UnitQuaternion {
    pub fn inner(&self) -> Quaternion {
        self.0
    }
}
