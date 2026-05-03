use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Dual(pub f32, pub f32);

impl Dual {
    pub fn sin(self) -> Self {
        Self(self.0.sin(), self.0.cos() * self.1)
    }

    pub fn cos(self) -> Self {
        Self(self.0.cos(), -self.0.sin() * self.1)
    }

    pub fn exp(self) -> Self {
        Self(self.0.exp(), self.0.exp() * self.1)
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.0 * other.1 + self.1 * other.0)
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(
            self.0 / other.0,
            (self.1 * other.0 - self.0 * other.1) / (other.0 * other.0),
        )
    }
}
