use crate::math;
use core::f32::consts::{FRAC_PI_2, PI, TAU};
use core::ops::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Angle(pub(crate) f32);

impl Angle {
    /// The 360° angle.
    pub const FULL_CIRCLE: Angle = Angle(TAU);
    /// The 180° angle.
    pub const HALF_CIRCLE: Angle = Angle(PI);
    /// The 90° angle.
    pub const QUARTER_CIRCLE: Angle = Angle(FRAC_PI_2);
    /// The 0° angle.
    pub const ZERO: Angle = Angle(0.);

    pub fn from_degrees(d: f32) -> Self {
        Self(d * PI / 180.0)
    }

    pub fn from_radians(r: f32) -> Self {
        Self(r)
    }

    pub fn abs(self) -> Self {
        Self(math::abs(self.0))
    }

    /// Normalize the angle to less than one full rotation (in the range 0..360 degrees).
    pub fn normalize(self) -> Self {
        Self(math::rem_euclid(self.0, 2.0 * PI))
    }

    pub fn to_degrees(self) -> f32 {
        180. * self.0 / PI
    }

    pub fn to_radians(self) -> f32 {
        self.0
    }

    pub fn sin(&self) -> f32 {
        math::sin(self.0)
    }

    pub fn cos(&self) -> f32 {
        math::cos(self.0)
    }

    pub fn tan(&self) -> f32 {
        math::tan(self.0)
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
