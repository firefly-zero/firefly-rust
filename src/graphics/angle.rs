use crate::*;
use core::f32::consts::{FRAC_PI_2, PI, TAU};
use core::ops::*;

/// An angle between two vectors.
///
/// Used by [`draw_arc`] and [`draw_sector`].
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

    /// An angle in degrees where 360.0 is the full circle.
    #[must_use]
    pub fn from_degrees(d: f32) -> Self {
        Self(d * PI / 180.0)
    }

    /// An angle in radians where [TAU] (doubled [PI]) is the full circle.
    #[must_use]
    pub fn from_radians(r: f32) -> Self {
        Self(r)
    }

    /// Convert the angle to an absolute (non-negative) value.
    #[must_use]
    pub fn abs(self) -> Self {
        Self(math::abs(self.0))
    }

    /// Normalize the angle to less than one full rotation (in the range 0°..360°).
    #[must_use]
    pub fn normalize(self) -> Self {
        Self(math::rem_euclid(self.0, TAU))
    }

    /// Get the angle value in degrees where 360.0 is the full circle..
    #[must_use]
    pub fn to_degrees(self) -> f32 {
        180. * self.0 / PI
    }

    /// Get the angle value in radians where [TAU] (doubled [PI]) is the full circle.
    #[must_use]
    pub fn to_radians(self) -> f32 {
        self.0
    }

    /// Approximates `sin(x)` of the angle with a maximum error of `0.002`.
    #[must_use]
    pub fn sin(&self) -> f32 {
        math::sin(self.0)
    }

    /// Approximates `cos(x)` of the angle with a maximum error of `0.002`.
    #[must_use]
    pub fn cos(&self) -> f32 {
        math::cos(self.0)
    }

    /// Approximates `tan(x)` of the angle with a maximum error of `0.6`.
    #[must_use]
    pub fn tan(&self) -> f32 {
        math::tan(self.0)
    }
}

impl Add for Angle {
    type Output = Self;

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
    type Output = Self;

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
    type Output = Self;

    fn neg(self) -> Self {
        Angle(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_degrees() {
        let d = Angle::from_degrees;
        let r = Angle::from_radians;
        assert_eq!(d(0.), r(0.));
        assert_eq!(d(180.), r(PI));
        assert_eq!(d(360.), r(TAU));
        assert_eq!(d(90.), r(PI / 2.));
    }

    #[test]
    fn test_abs() {
        let a = Angle::from_degrees;
        assert_eq!(a(90.), a(90.));
        assert_ne!(a(-90.), a(90.));
        assert_eq!(a(-90.).abs(), a(90.));
    }

    #[test]
    fn test_normalize() {
        let a = Angle::from_degrees;
        assert_eq!(a(90.).normalize(), a(90.));
        assert_eq!(a(360.).normalize(), a(0.));
        assert_eq!(a(-90.).normalize(), a(270.));
    }

    #[test]
    fn test_to_degrees() {
        let a = Angle::from_degrees;
        assert_eq!(a(47.).to_degrees(), 47.);
        assert_eq!(a(0.).to_degrees(), 0.);
        assert_eq!(a(90.).to_degrees(), 90.);
        assert_eq!(a(370.).to_degrees(), 370.);
    }
}
