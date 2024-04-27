use core::f32::consts::{FRAC_1_PI, PI};

const SIGN_MASK: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

/// Approximates `tan(x)` in radians with a maximum error of `0.6`.
pub fn tan(x: f32) -> f32 {
    sin(x) / cos(x)
}

/// Approximates `sin(x)` in radians with a maximum error of `0.002`.
pub fn sin(x: f32) -> f32 {
    cos(x - PI / 2.0)
}

/// Approximates `cos(x)` in radians with a maximum error of `0.002`.
pub fn cos(x: f32) -> f32 {
    let mut x = x;
    x *= FRAC_1_PI / 2.0;
    x -= 0.25 + floor(x + 0.25);
    x *= 16.0 * (abs(x) - 0.5);
    x += 0.225 * x * (abs(x) - 1.0);
    x
}

/// Returns the largest integer less than or equal to a number.
pub fn floor(x: f32) -> f32 {
    let mut res = (x as i32) as f32;
    if x < res {
        res -= 1.0;
    }
    res
}

/// Returns the absolute value of a number.
pub fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & !SIGN_MASK)
}

/// Approximates the square root of a number with an average deviation of ~5%.
pub fn sqrt(x: f32) -> f32 {
    if x >= 0. {
        f32::from_bits((x.to_bits() + 0x3f80_0000) >> 1)
    } else {
        f32::NAN
    }
}
