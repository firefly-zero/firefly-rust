use crate::*;

/// A loaded image file.
///
/// Can be loaded as [`FileBuf`] from ROM with [`load_file_buf`]
/// and then cast using [Into].
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

impl<'a> Image<'a> {
    /// Get a rectangle subregion of the image.
    #[must_use]
    pub fn sub(&self, p: Point, s: Size) -> SubImage<'a> {
        SubImage {
            point: p,
            size: s,
            raw: self.raw,
        }
    }
}

/// A subregion of an image. Constructed using [`Image::sub`].
#[expect(clippy::module_name_repetitions)]
pub struct SubImage<'a> {
    pub(crate) point: Point,
    pub(crate) size: Size,
    pub(crate) raw: &'a [u8],
}
