use crate::*;

/// A loaded image file.
///
/// Can be loaded as [`FileBuf`] from ROM with [`load_file_buf`]
/// and then cast using [`Into`].
pub struct Image<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> From<File<'a>> for Image<'a> {
    fn from(value: File<'a>) -> Self {
        Self { raw: value.raw }
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a FileBuf> for Image<'a> {
    fn from(value: &'a FileBuf) -> Self {
        Self { raw: &value.raw }
    }
}

impl<'a> From<Canvas<'a>> for Image<'a> {
    fn from(value: Canvas<'a>) -> Self {
        Self { raw: value.raw }
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a CanvasBuf> for Image<'a> {
    fn from(value: &'a CanvasBuf) -> Self {
        Self { raw: &value.raw }
    }
}

impl<'a> Image<'a> {
    /// Reinterpret raw bytes as an image.
    ///
    /// # Safety
    ///
    /// Using this function requires a good understanding of the internal
    /// Firefly Zero binary image format. In 99% cases, you should not construct
    /// a raw image but instead load it from a [`File`] or generate using [`Canvas`].
    #[must_use]
    pub const unsafe fn from_bytes(raw: &'a [u8]) -> Self {
        Self { raw }
    }

    /// Get a rectangle subregion of the image.
    #[must_use]
    pub const fn sub(&self, p: Point, s: Size) -> SubImage<'a> {
        SubImage {
            point: p,
            size: s,
            raw: self.raw,
        }
    }

    /// The color used for transparency. If no transparency, returns [`Color::None`].
    #[must_use]
    pub fn transparency(&self) -> Color {
        Color::from(self.raw[3] + 1)
    }

    /// The number of pixels the image has.
    #[must_use]
    pub const fn pixels(&self) -> usize {
        const HEADER_SIZE: usize = 4;
        const PPB: usize = 2;
        (self.raw.len() - HEADER_SIZE) * PPB
    }

    /// The image width in pixels.
    #[must_use]
    pub fn width(&self) -> u16 {
        let big = u16::from(self.raw[1]);
        let little = u16::from(self.raw[2]);
        big | (little << 8)
    }

    /// The image height in pixels.
    #[must_use]
    pub fn height(&self) -> u16 {
        let p = self.pixels();
        let w = usize::from(self.width());
        p.checked_div(w).unwrap_or(0) as u16
    }

    /// The image size in pixels.
    #[must_use]
    pub fn size(&self) -> Size {
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
