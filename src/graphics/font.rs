use crate::*;

/// A loaded font file.
///
/// Can be loaded as [`FileBuf`] from ROM with [`rom::load_buf`]
/// and then cast using [Into].
pub struct Font<'a> {
    pub(crate) raw: &'a [u8],
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
