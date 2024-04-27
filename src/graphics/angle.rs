use core::f32::consts::PI;
use core::ops::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Angle(pub(crate) i32);

impl Angle {
    pub fn from_degrees(d: i32) -> Self {
        Self(d)
    }

    pub fn from_radians(r: f32) -> Self {
        let d = r * 180. / PI;
        Self(d as i32)
    }

    /// Normalize the angle to less than one full rotation (ie. in the range 0..360).
    pub fn normalize(self) -> Self {
        Self(self.0.rem_euclid(360))
    }

    pub fn as_degrees(&self) -> i32 {
        self.0
    }

    pub fn as_radians(&self) -> f32 {
        self.0 as f32 * PI / 180.
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
