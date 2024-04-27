//! A few useful math (trigonometric) functions for [f32].
//!
//! Use [micromath] and [nalgebra] if you need more.
//!
//! [micromath]: https://github.com/tarcieri/micromath
//! [nalgebra]: https://github.com/dimforge/nalgebra

use core::f32::consts::{FRAC_1_PI, FRAC_PI_2, PI};

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
    // https://github.com/tarcieri/micromath/blob/main/src/float/cos.rs
    let mut x = x;
    x *= FRAC_1_PI / 2.0;
    x -= 0.25 + floor(x + 0.25);
    x *= 16.0 * (abs(x) - 0.5);
    x += 0.225 * x * (abs(x) - 1.0);
    x
}

/// Returns the largest integer less than or equal to a number.
pub fn floor(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/floor.rs
    let mut res = (x as i32) as f32;
    if x < res {
        res -= 1.0;
    }
    res
}

/// Returns the absolute value of a number.
pub fn abs(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/abs.rs
    f32::from_bits(x.to_bits() & !SIGN_MASK)
}

/// Approximates the square root of a number with an average deviation of ~5%.
pub fn sqrt(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/sqrt.rs
    if x >= 0. {
        f32::from_bits((x.to_bits() + 0x3f80_0000) >> 1)
    } else {
        f32::NAN
    }
}

// Calculates the least nonnegative remainder of lhs (mod rhs).
pub(crate) fn rem_euclid(lhs: f32, rhs: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/rem_euclid.rs
    let r = lhs % rhs;
    if r < 0.0 {
        r + abs(rhs)
    } else {
        r
    }
}

/// Approximates `atan(x)` approximation in radians with a maximum error of
/// `0.002`.
///
/// Returns [`f32::NAN`] if the number is [`f32::NAN`].
pub fn atan(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/atan.rs
    FRAC_PI_2 * atan_norm(x)
}

/// Approximates `atan(x)` normalized to the `[−1,1]` range with a maximum
/// error of `0.1620` degrees.
pub fn atan_norm(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/atan.rs
    const SIGN_MASK: u32 = 0x8000_0000;
    const B: f32 = 0.596_227;

    // Extract the sign bit
    let ux_s = SIGN_MASK & x.to_bits();

    // Calculate the arctangent in the first quadrant
    let bx_a = abs(B * x);
    let n = bx_a + x * x;
    let atan_1q = n / (1.0 + bx_a + n);

    // Restore the sign bit and convert to float
    f32::from_bits(ux_s | atan_1q.to_bits())
}
