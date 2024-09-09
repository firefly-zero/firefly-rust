use super::*;

pub struct Time(u32);

impl Time {
    #[must_use]
    pub fn samples(s: u32) -> Self {
        Self(s)
    }

    #[must_use]
    pub fn seconds(s: u32) -> Self {
        Self(s * SAMPLE_RATE)
    }

    #[must_use]
    pub fn ms(s: u32) -> Self {
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
