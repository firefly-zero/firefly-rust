use super::*;

pub struct Time(pub(super) u32);

impl Time {
    pub const ZERO: Self = Self(0);
    pub const SECOND: Self = Self(SAMPLE_RATE);

    #[must_use]
    pub fn from_samples(s: u32) -> Self {
        Self(s)
    }

    #[must_use]
    pub fn from_seconds(s: u32) -> Self {
        Self(s * SAMPLE_RATE)
    }

    #[must_use]
    pub fn from_ms(s: u32) -> Self {
        Self(s * SAMPLE_RATE / 1000)
    }

    #[must_use]
    pub fn duration(s: core::time::Duration) -> Self {
        let s = s.as_secs_f32() * 44_100.;
        #[expect(clippy::cast_sign_loss)]
        let s = s as u32;
        Self(s)
    }
}

/// Alias for [`Time::from_samples`].
#[must_use]
pub fn samples(s: u32) -> Time {
    Time::from_samples(s)
}

/// Alias for [`Time::from_seconds`].
#[must_use]
pub fn seconds(s: u32) -> Time {
    Time::from_seconds(s)
}

/// Alias for [`Time::from_ms`].
#[must_use]
pub fn ms(s: u32) -> Time {
    Time::from_ms(s)
}

impl core::ops::Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl core::ops::Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl core::ops::Div<u32> for Time {
    type Output = Time;

    fn div(self, rhs: u32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl core::ops::Mul<u32> for Time {
    type Output = Time;

    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
