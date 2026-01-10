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

    /// Bits per pixel. One of: 1, 2, or 4.
    #[must_use]
    pub const fn bpp(&self) -> u8 {
        self.raw[1]
    }

    /// The color used for transparency. If no transparency, returns [`Color::None`].
    #[must_use]
    pub fn transparency(&self) -> Color {
        Color::from(self.raw[4] + 1)
    }

    // pub fn set_transparency(&mut self, c: Color) {
    //     let c: i32 = c.into();
    //     if c == 0 {
    //         self.raw[4] = 16;
    //     } else {
    //         self.raw[4] = c as u8;
    //     }
    // }

    /// The number of pixels the image has.
    #[must_use]
    pub const fn pixels(&self) -> usize {
        self.raw.len() * 8 / self.bpp() as usize
    }

    /// The image width in pixels.
    #[must_use]
    pub fn width(&self) -> u16 {
        let big = u16::from(self.raw[2]);
        let little = u16::from(self.raw[3]);
        big | (little << 8)
    }

    /// The image height in pixels.
    #[must_use]
    pub fn height(&self) -> u16 {
        let p = self.pixels();
        let w = self.width() as usize;
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

    /// Get the color used to represent the given pixel value.
    #[must_use]
    pub fn get_color(&self, p: u8) -> Color {
        if p > 15 {
            return Color::None;
        }
        let byte_idx = usize::from(5 + p / 2);
        let mut byte_val = self.raw[byte_idx];
        if p.is_multiple_of(2) {
            byte_val >>= 4;
        }
        byte_val &= 0b1111;
        let transp = self.raw[4];
        if byte_val == transp {
            return Color::None;
        }
        Color::from(byte_val + 1)
    }
}

/// A subregion of an image. Constructed using [`Image::sub`].
pub struct SubImage<'a> {
    pub(crate) point: Point,
    pub(crate) size: Size,
    pub(crate) raw: &'a [u8],
}
