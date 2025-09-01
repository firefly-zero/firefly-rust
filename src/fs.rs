//! Access file system: the game ROM files and the data dir.

use crate::graphics::*;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec;

/// Like [File] but owns the buffer.
///
/// Returned by [`rom::load_buf`] and [`data::load_buf`]. Requires a global allocator.
/// For a file of statically-known size, you might want to use [`rom::load`]
/// and [`data::load`] instead.
#[cfg(feature = "alloc")]
pub struct FileBuf {
    pub(crate) raw: Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl FileBuf {
    /// Access the raw data in the file.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.raw
    }

    /// Interpret the file as a font.
    #[must_use]
    pub fn as_font(&'_ self) -> Font<'_> {
        Font { raw: &self.raw }
    }

    /// Interpret the file as an image.
    #[must_use]
    pub fn as_image(&'_ self) -> Image<'_> {
        Image { raw: &self.raw }
    }

    #[must_use]
    pub fn into_vec(self) -> alloc::vec::Vec<u8> {
        self.raw.into_vec()
    }
}

#[cfg(feature = "alloc")]
impl From<FileBuf> for Box<[u8]> {
    fn from(value: FileBuf) -> Self {
        value.raw
    }
}

#[cfg(feature = "alloc")]
impl From<FileBuf> for alloc::vec::Vec<u8> {
    fn from(value: FileBuf) -> Self {
        value.into_vec()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<FileBuf> for alloc::string::String {
    type Error = alloc::string::FromUtf8Error;

    fn try_from(value: FileBuf) -> Result<Self, Self::Error> {
        let v = value.into_vec();
        alloc::string::String::from_utf8(v)
    }
}

/// A file loaded from ROM or data dir into the memory.
///
/// Returned by [`rom::load`] and [`data::load`] which requires a pre-allocated buffer
/// of the right size. If the file size is deterimed dynamically,
/// you might want to use [`rom::load_buf`] and [`data::load_buf`] instead
/// (which will take care of the dynamic allocation).
pub struct File<'a> {
    pub(crate) raw: &'a [u8],
}

impl File<'_> {
    #[must_use]
    pub const fn data(&self) -> &[u8] {
        self.raw
    }

    #[must_use]
    pub const fn as_font(&'_ self) -> Font<'_> {
        Font { raw: self.raw }
    }

    #[must_use]
    pub const fn as_image(&'_ self) -> Image<'_> {
        Image { raw: self.raw }
    }
}

/// Get a file size in the data dir.
///
/// If the file does not exist, 0 is returned.
#[must_use]
pub fn get_file_size(name: &str) -> usize {
    let path_ptr = name.as_ptr();
    let size = unsafe { bindings::get_file_size(path_ptr as u32, name.len() as u32) };
    size as usize
}

/// Read the whole file with the given name into the given buffer.
///
/// If the file size is not known in advance (and so the buffer has to be allocated
/// dynamically), consider using [`load_file_buf`] instead.
pub fn load_file<'a>(name: &str, buf: &'a mut [u8]) -> File<'a> {
    let path_ptr = name.as_ptr();
    let buf_ptr = buf.as_mut_ptr();
    unsafe {
        bindings::load_file(
            path_ptr as u32,
            name.len() as u32,
            buf_ptr as u32,
            buf.len() as u32,
        );
    }
    File { raw: buf }
}

/// Read the whole file with the given name.
///
/// If you have a pre-allocated buffer of the right size, use [`load_file`] instead.
///
/// `None` is returned if the file does not exist.
#[cfg(feature = "alloc")]
#[must_use]
pub fn load_file_buf(name: &str) -> Option<FileBuf> {
    let size = get_file_size(name);
    if size == 0 {
        return None;
    }
    let mut buf = vec![0; size];
    load_file(name, &mut buf);
    Some(FileBuf {
        raw: buf.into_boxed_slice(),
    })
}

/// Write the buffer into the given file in the data dir.
///
/// If the file exists, it will be overwritten.
/// If it doesn't exist, it will be created.
pub fn dump_file(name: &str, buf: &[u8]) {
    let path_ptr = name.as_ptr();
    let buf_ptr = buf.as_ptr();
    unsafe {
        bindings::dump_file(
            path_ptr as u32,
            name.len() as u32,
            buf_ptr as u32,
            buf.len() as u32,
        );
    }
}

/// Remove file (if exists) with the given name from the data dir.
pub fn remove_file(name: &str) {
    let path_ptr = name.as_ptr();
    unsafe {
        bindings::remove_file(path_ptr as u32, name.len() as u32);
    }
}

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

mod bindings {
    #[link(wasm_import_module = "fs")]
    extern "C" {
        pub(crate) fn get_file_size(path_ptr: u32, path_len: u32) -> u32;
        pub(crate) fn load_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
        pub(crate) fn dump_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
        pub(crate) fn remove_file(path_ptr: u32, path_len: u32);
    }
}
