use crate::math::rem_euclid;
use core::f32::consts::PI;
use core::ops::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Angle(pub(crate) f32);

impl Angle {
    pub fn from_degrees(d: f32) -> Self {
        Self(d * PI / 180.0)
    }

    pub fn from_radians(r: f32) -> Self {
        Self(r)
    }

    /// Normalize the angle to less than one full rotation (ie. in the range 0..360).
    pub fn normalize(self) -> Self {
        Self(rem_euclid(self.0, 2.0 * PI))
    }

    pub fn as_degrees(&self) -> f32 {
        180. * self.0 / PI
    }

    pub fn as_radians(&self) -> f32 {
        self.0
    }

    pub fn into_degrees(self) -> f32 {
        180. * self.0 / PI
    }

    pub fn into_radians(self) -> f32 {
        self.0
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Self {
        Angle(self.0 + other.0)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, other: Angle) {
        self.0 += other.0;
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Self {
        Angle(self.0 - other.0)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Angle) {
        self.0 -= other.0;
    }
}

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Self {
        Angle(-self.0)
    }
}
