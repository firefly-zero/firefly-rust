//! A few useful math (trigonometric) functions for [f32].
//!
//! Use [micromath] and [nalgebra] if you need more.
//!
//! [micromath]: https://github.com/tarcieri/micromath
//! [nalgebra]: https://github.com/dimforge/nalgebra

use core::f32::consts::{FRAC_1_PI, PI};

const SIGN_MASK: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;

/// Approximates `tan(x)` in radians with a maximum error of `0.6`.
#[must_use]
pub fn tan(x: f32) -> f32 {
    sin(x) / cos(x)
}

/// Approximates `sin(x)` in radians with a maximum error of `0.002`.
#[must_use]
pub fn sin(x: f32) -> f32 {
    cos(x - PI / 2.0)
}

/// Approximates `cos(x)` in radians with a maximum error of `0.002`.
#[must_use]
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
#[must_use]
pub fn floor(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/floor.rs
    #[expect(clippy::cast_precision_loss)]
    let mut res = (x as i32) as f32;
    if x < res {
        res -= 1.0;
    }
    res
}

/// Returns the absolute value of a number.
#[must_use]
pub fn abs(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/abs.rs
    f32::from_bits(x.to_bits() & !SIGN_MASK)
}

/// Approximates the square root of a number with an average deviation of ~5%.
#[must_use]
pub fn sqrt(x: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/sqrt.rs
    if x >= 0. {
        f32::from_bits((x.to_bits() + 0x3f80_0000) >> 1)
    } else {
        f32::NAN
    }
}

// Calculates the least nonnegative remainder of lhs (mod rhs).
#[must_use]
pub fn rem_euclid(lhs: f32, rhs: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/rem_euclid.rs
    let r = lhs % rhs;
    if r < 0.0 { r + abs(rhs) } else { r }
}

#[must_use]
pub fn atan2(lhs: f32, rhs: f32) -> f32 {
    // https://github.com/tarcieri/micromath/blob/main/src/float/atan2.rs
    let n = atan2_norm(lhs, rhs);
    PI / 2.0 * if n > 2.0 { n - 4.0 } else { n }
}

/// Approximates `atan2(y,x)` normalized to the `[0, 4)` range with a maximum
/// error of `0.1620` degrees.
#[must_use]
#[expect(clippy::similar_names)]
pub(crate) fn atan2_norm(lhs: f32, rhs: f32) -> f32 {
    const SIGN_MASK: u32 = 0x8000_0000;
    const B: f32 = 0.596_227;

    let y = lhs;
    let x = rhs;

    // Extract sign bits from floating point values
    let ux_s = SIGN_MASK & x.to_bits();
    let uy_s = SIGN_MASK & y.to_bits();

    // Determine quadrant offset
    #[expect(clippy::cast_precision_loss)]
    let q = ((!ux_s & uy_s) >> 29 | ux_s >> 30) as f32;

    // Calculate arctangent in the first quadrant
    let bxy_a = (B * x * y).abs();
    let n = bxy_a + y * y;
    let atan_1q = n / (x * x + bxy_a + n);

    // Translate it to the proper quadrant
    let uatan_2q = (ux_s ^ uy_s) | atan_1q.to_bits();
    q + f32::from_bits(uatan_2q)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(clippy::float_cmp)]
    fn test_sqrt() {
        assert_eq!(sqrt(4.), 2.);
        assert_eq!(sqrt(9.), 3.125);
    }
}
