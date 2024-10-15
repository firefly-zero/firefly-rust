use crate::*;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec;

/// Canvas is an [`Image`] that can be drawn upon.
///
/// [`CanvasBuf`] is the same as [`Canvas`] but holds the ownership of the underlying slice.
#[cfg(feature = "alloc")]
#[expect(clippy::module_name_repetitions)]
pub struct CanvasBuf {
    pub(crate) raw: Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl CanvasBuf {
    /// Create new empty canvas.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn new(s: Size) -> Self {
        const HEADER_SIZE: usize = 5 + 8;
        let body_size = s.width * s.height / 2;
        let mut raw = vec![0; HEADER_SIZE + body_size as usize];
        prepare_slice(&mut raw, s.width);
        Self {
            raw: raw.into_boxed_slice(),
        }
    }

    /// Represent the canvas as an [`Image`].
    #[must_use]
    pub fn as_image(&self) -> Image<'_> {
        Image { raw: &self.raw }
    }
}

/// Canvas is an [`Image`] that can be drawn upon.
pub struct Canvas<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> Canvas<'a> {
    /// Create new empty canvas using the given slice.
    ///
    /// Returns [`None`] if the slice is too small for the given image size.
    /// A bigger slice than needed is fine.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn new(s: Size, raw: &'a mut [u8]) -> Option<Self> {
        const HEADER_SIZE: usize = 5 + 8;
        let body_size = s.width * s.height / 2;
        let exp_size = HEADER_SIZE + body_size as usize;
        if raw.len() < exp_size {
            return None;
        }
        prepare_slice(raw, s.width);
        Some(Self {
            raw: &raw[..exp_size],
        })
    }

    /// Represent the canvas as an [`Image`].
    #[must_use]
    pub fn as_image(&self) -> Image<'a> {
        Image { raw: self.raw }
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a CanvasBuf> for Canvas<'a> {
    fn from(value: &'a CanvasBuf) -> Self {
        Self { raw: &value.raw }
    }
}

#[expect(clippy::cast_sign_loss)]
fn prepare_slice(raw: &mut [u8], width: i32) {
    raw[0] = 0x21; // magic number
    raw[1] = 4; // BPP
    raw[2] = width as u8; // width
    raw[3] = (width >> 8) as u8; // width
    raw[4] = 255; // transparency

    // color swaps
    for i in 0u8..8u8 {
        raw[5 + i as usize] = ((i * 2) << 4) | (i * 2 + 1);
    }
}
