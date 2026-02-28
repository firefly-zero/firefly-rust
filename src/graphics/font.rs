use crate::*;

/// A loaded font file.
///
/// Can be loaded as [`FileBuf`] from ROM with [`load_file_buf`]
/// and then cast using [`FileBuf::as_font`].
pub struct Font<'a> {
    pub(crate) raw: &'a [u8],
}

impl Font<'_> {
    /// Check if the font encoding is ASCII.
    #[must_use]
    pub fn is_ascii(&self) -> bool {
        self.raw[1] == 0
    }

    /// Calculate width (in pixels) of the given ASCII text.
    ///
    /// This function does not account for newlines.
    #[must_use]
    pub fn line_width_ascii(&self, t: &str) -> u32 {
        t.len() as u32 * u32::from(self.char_width())
    }

    /// Calculate width (in pixels) of the given UTF-8 text.
    ///
    /// This function does not account for newlines.
    #[must_use]
    pub fn line_width_utf8(&self, t: &str) -> u32 {
        t.chars().count() as u32 * u32::from(self.char_width())
    }

    /// The width (in pixels) of one character.
    #[must_use]
    pub fn char_width(&self) -> u8 {
        self.raw[2]
    }

    /// The hight (in pixels) of one character (one line).
    #[must_use]
    pub fn char_height(&self) -> u8 {
        self.raw[3]
    }
}

impl<'a> From<File<'a>> for Font<'a> {
    fn from(value: File<'a>) -> Self {
        Self { raw: value.raw }
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a FileBuf> for Font<'a> {
    fn from(value: &'a FileBuf) -> Self {
        Self { raw: &value.raw }
    }
}
