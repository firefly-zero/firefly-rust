use crate::*;

/// Owned version of [`Image`].
///
/// Can be loaded as [`FileBuf`] from ROM with [`load_file_buf`]
/// and then cast using [`Into::into`].
#[cfg(feature = "alloc")]
pub struct ImageBuf {
    pub(crate) raw: alloc::boxed::Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl ImageBuf {
    /// Create new empty image.
    ///
    /// The given `transp` color is used as the transparency color.
    /// if [`Color::None`] is given, then the image will not have transparency
    /// and it will be filled with [`Color::Black`] by default.
    ///
    /// Use [`set_canvas`] (or [`with_canvas`]) and the drawing functions
    /// to fill the newly created image.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn new(s: Size, transp: Color) -> Self {
        const HEADER_SIZE: usize = 4;
        let body_size = s.width * s.height / 2;
        let t = transp as u8 - 1;
        let def = if t == 255 { 0 } else { t };
        let mut raw = vec![def; HEADER_SIZE + body_size as usize];
        prepare_slice(&mut raw, s.width, t);
        Self {
            raw: raw.into_boxed_slice(),
        }
    }
}

#[expect(clippy::cast_sign_loss)]
const fn prepare_slice(raw: &mut [u8], width: i32, t: u8) {
    raw[0] = 0x22; // magic number
    raw[1] = width as u8; // width
    raw[2] = (width >> 8) as u8; // width
    raw[3] = t; // transparency
}

#[cfg(feature = "alloc")]
impl Image for ImageBuf {
    unsafe fn as_bytes(&self) -> &[u8] {
        &self.raw
    }
}

#[cfg(feature = "alloc")]
impl From<FileBuf> for ImageBuf {
    fn from(value: FileBuf) -> Self {
        Self { raw: value.raw }
    }
}

/// A loaded image file.
///
/// Can be loaded as [`FileRef`] from ROM with [`load_file`]
/// and then cast using [`Into`].
pub struct ImageRef<'a> {
    raw: &'a [u8],
}

impl<'a> ImageRef<'a> {
    /// Create new empty image using the given slice.
    ///
    /// Returns [`None`] if the slice is too small for the given image size.
    /// A bigger slice than needed is fine.
    ///
    /// The given `transp` color is used as the transparency color.
    /// if [`Color::None`] is given, then the image will not have transparency
    /// and it will be filled with [`Color::Black`] by default.
    ///
    /// Use [`set_canvas`] (or [`with_canvas`]) and the drawing functions
    /// to fill the newly created image.
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn new(s: Size, raw: &'a mut [u8], transp: Color) -> Option<Self> {
        const HEADER_SIZE: usize = 4;
        let body_size = s.width * s.height / 2;
        let exp_size = HEADER_SIZE + body_size as usize;
        if raw.len() < exp_size {
            return None;
        }
        let t = transp as u8 - 1;
        raw.fill(t);
        prepare_slice(raw, s.width, t);
        Some(Self {
            raw: &raw[..exp_size],
        })
    }

    /// Reinterpret raw bytes as an image.
    ///
    /// # Safety
    ///
    /// Using this function requires a good understanding of the internal
    /// Firefly Zero binary image format. In 99% cases, you should not construct
    /// a raw image but instead load it from a [`FileRef`] or generate using [`ImageRef::new`].
    #[must_use]
    pub unsafe fn from_bytes(raw: &'a [u8]) -> Self {
        Self { raw }
    }
}

impl Image for ImageRef<'_> {
    unsafe fn as_bytes(&self) -> &[u8] {
        self.raw
    }
}

impl<'a> From<FileRef<'a>> for ImageRef<'a> {
    fn from(value: FileRef<'a>) -> Self {
        Self { raw: value.raw }
    }
}

/// A loaded image file.
pub trait Image {
    /// Get the raw image representation.
    ///
    /// # Safety
    ///
    /// Don't use it. The internal image representation might change
    /// in the future and so should not be relied upon.
    unsafe fn as_bytes(&self) -> &[u8];

    /// Get a rectangle subregion of the image.
    #[must_use]
    fn sub(&self, p: Point, s: Size) -> SubImage<'_> {
        let raw = unsafe { self.as_bytes() };
        SubImage {
            point: p,
            size: s,
            raw,
        }
    }

    /// The color used for transparency. If no transparency, returns [`Color::None`].
    #[must_use]
    fn transparency(&self) -> Color {
        let raw = unsafe { self.as_bytes() };
        Color::from(raw[3] + 1)
    }

    /// The number of pixels the image has.
    #[must_use]
    fn pixels(&self) -> usize {
        const HEADER_SIZE: usize = 4;
        const PPB: usize = 2;
        let raw = unsafe { self.as_bytes() };
        (raw.len() - HEADER_SIZE) * PPB
    }

    /// The image width in pixels.
    #[must_use]
    fn width(&self) -> u16 {
        let raw = unsafe { self.as_bytes() };
        let big = u16::from(raw[1]);
        let little = u16::from(raw[2]);
        big | (little << 8)
    }

    /// The image height in pixels.
    #[must_use]
    fn height(&self) -> u16 {
        let p = self.pixels();
        let w = usize::from(self.width());
        p.checked_div(w).unwrap_or(0) as u16
    }

    /// The image size in pixels.
    #[must_use]
    fn size(&self) -> Size {
        Size {
            width: i32::from(self.width()),
            height: i32::from(self.height()),
        }
    }
}

/// A subregion of an image. Constructed using [`Image::sub`].
pub struct SubImage<'a> {
    pub(crate) point: Point,
    pub(crate) size: Size,
    pub(crate) raw: &'a [u8],
}
