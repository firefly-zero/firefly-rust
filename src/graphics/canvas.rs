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
        raw[0] = 0x21; // magic number
        raw[1] = 4; // BPP
        raw[2] = (s.width) as u8; // width
        raw[3] = (s.width >> 8) as u8; // width
        raw[4] = 255; // transparency

        // color swaps
        for i in 0u8..8u8 {
            raw[5 + i as usize] = ((i * 2) << 4) | (i * 2 + 1);
        }

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
