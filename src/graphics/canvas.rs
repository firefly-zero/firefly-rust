use crate::*;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec;

// /// Canvas is an [`Image`] that can be drawn upon.
// #[cfg(feature = "alloc")]
// pub struct CanvasStatic<const SIZE: usize> {
//     pub(crate) raw: [u8; SIZE],
// }

// impl<const SIZE: usize> CanvasStatic<SIZE> {
//     /// Create new [`CanvasStatic`].
//     #[must_use]
//     pub const fn new(width: usize) -> Self {
//         const HEADER_SIZE: usize = 4;
//         let mut raw = [0u8; SIZE];
//         #[expect(clippy::cast_possible_wrap)]
//         prepare_slice(&mut raw, width as i32);
//         Self { raw }
//     }
// }

/// Canvas is an [`Image`] that can be drawn upon.
///
/// [`CanvasBuf`] is the same as [`Canvas`] but holds the ownership of the underlying slice.
#[cfg(feature = "alloc")]
pub struct CanvasBuf {
    pub(crate) raw: Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl CanvasBuf {
    /// Create new empty canvas.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn new(s: Size) -> Self {
        const HEADER_SIZE: usize = 4;
        let body_size = s.width * s.height / 2;
        let mut raw = vec![0; HEADER_SIZE + body_size as usize];
        prepare_slice(&mut raw, s.width);
        Self {
            raw: raw.into_boxed_slice(),
        }
    }
}

impl Canvas for CanvasBuf {
    unsafe fn as_bytes(&self) -> &[u8] {
        &self.raw
    }
}

/// Canvas is an [`Image`] that can be drawn upon.
pub struct CanvasRef<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> CanvasRef<'a> {
    /// Create new empty canvas using the given slice.
    ///
    /// Returns [`None`] if the slice is too small for the given image size.
    /// A bigger slice than needed is fine.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn new(s: Size, raw: &'a mut [u8]) -> Option<Self> {
        const HEADER_SIZE: usize = 4;
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
}

impl Canvas for CanvasRef<'_> {
    unsafe fn as_bytes(&self) -> &[u8] {
        self.raw
    }
}

/// Canvas is an [`Image`] that can be drawn upon.
pub trait Canvas {
    /// Get the raw canvas representation.
    ///
    /// # Safety
    ///
    /// Don't use it. The internal canvas representation might change
    /// in the future and so should not be relied upon.
    unsafe fn as_bytes(&self) -> &[u8];
}

#[expect(clippy::cast_sign_loss)]
const fn prepare_slice(raw: &mut [u8], width: i32) {
    raw[0] = 0x22; // magic number
    raw[1] = width as u8; // width
    raw[2] = (width >> 8) as u8; // width
    raw[3] = 255; // transparency
}
