use crate::*;

/// Owned version of [`Font`].
///
/// Can be loaded as [`FileBuf`] from ROM with [`load_file_buf`]
/// and then cast using [`Into::into`].
#[cfg(feature = "alloc")]
pub struct FontBuf {
    pub(crate) raw: alloc::boxed::Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl Font for FontBuf {
    unsafe fn as_bytes(&self) -> &[u8] {
        &self.raw
    }
}

#[cfg(feature = "alloc")]
impl From<FileBuf> for FontBuf {
    fn from(value: FileBuf) -> Self {
        Self { raw: value.raw }
    }
}

/// Borrowed version of [`Font`].
pub struct FontRef<'a> {
    raw: &'a [u8],
}

impl Font for FontRef<'_> {
    unsafe fn as_bytes(&self) -> &[u8] {
        self.raw
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<FileRef<'a>> for FontRef<'a> {
    fn from(value: FileRef<'a>) -> Self {
        Self { raw: value.raw }
    }
}

/// A loaded font file.
pub trait Font {
    /// Get the raw font file representation.
    ///
    /// # Safety
    ///
    /// Don't use it. The internal font representation might change
    /// in the future and so should not be relied upon.
    unsafe fn as_bytes(&self) -> &[u8];

    /// Check if the font encoding is ASCII.
    #[must_use]
    fn is_ascii(&self) -> bool {
        let raw = unsafe { self.as_bytes() };
        raw[1] == 0
    }

    /// Calculate width (in pixels) of the given ASCII text.
    ///
    /// This function does not account for newlines.
    #[must_use]
    fn line_width_ascii(&self, t: &str) -> u32 {
        t.len() as u32 * u32::from(self.char_width())
    }

    /// Calculate width (in pixels) of the given UTF-8 text.
    ///
    /// This function does not account for newlines.
    #[must_use]
    fn line_width_utf8(&self, t: &str) -> u32 {
        t.chars().count() as u32 * u32::from(self.char_width())
    }

    /// The width (in pixels) of one glyph bounding box.
    #[must_use]
    fn char_width(&self) -> u8 {
        let raw = unsafe { self.as_bytes() };
        raw[2]
    }

    /// The height (in pixels) of one glyph (one line) bounding box.
    #[must_use]
    fn char_height(&self) -> u8 {
        let raw = unsafe { self.as_bytes() };
        raw[3]
    }

    /// Offset from the top of the glyph bounding box to the baseline.
    #[must_use]
    fn baseline(&self) -> u8 {
        let raw = unsafe { self.as_bytes() };
        raw[4]
    }
}
