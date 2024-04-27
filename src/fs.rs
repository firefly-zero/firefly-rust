//! Access file system: the game ROM files and the data dir.

use crate::graphics::{Point, Size};
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Like [File] but owns the buffer.
///
/// Returned by [rom::load_buf] and [data::load_buf]. Requires a global allocator.
/// For a file of statically-known size, you might want to use [rom::load]
/// and [data::load] instead.
#[cfg(feature = "alloc")]
pub struct FileBuf {
    pub(crate) raw: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl FileBuf {
    /// Access the raw data in the file.
    pub fn data(&self) -> &[u8] {
        &self.raw
    }

    /// Interpret the file as a font.
    pub fn as_font(&self) -> Font {
        Font { raw: &self.raw }
    }

    /// Interpret the file as an image.
    pub fn as_image(&self) -> Image {
        Image { raw: &self.raw }
    }
}

/// A file loaded from ROM or data dir into the memory.
///
/// Returned by [rom::load] and [data::load] which requires a pre-allocated buffer
/// of the right size. If the file size is deterimed dynamically,
/// you might want to use [rom::load_buf] and [data::load_buf] instead
/// (which will take care of the dynamic allocation).
pub struct File<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> File<'a> {
    pub fn data(&self) -> &[u8] {
        self.raw
    }

    pub fn as_font(&self) -> Font {
        Font { raw: self.raw }
    }

    pub fn as_image(&self) -> Image {
        Image { raw: self.raw }
    }
}

/// Functions for accessing files in the app ROM.
pub mod rom {
    use super::*;
    use crate::bindings as b;

    /// Determine the required size (in bytes) to store the given file.
    ///
    /// If the file does not exist, 0 is returned.
    #[must_use]
    pub fn get_size(name: &str) -> usize {
        let path_ptr = name.as_ptr();
        let size = unsafe { b::get_rom_file_size(path_ptr as u32, name.len() as u32) };
        size as usize
    }

    /// Read the whole file with the given name into the given buffer.
    ///
    /// If the file size is not known in advance (and so the buffer has to be allocated
    /// dynamically), consider using [load_buf] instead.
    pub fn load<'a>(name: &str, buf: &'a mut [u8]) -> File<'a> {
        let path_ptr = name.as_ptr();
        let buf_ptr = buf.as_mut_ptr();
        unsafe {
            b::load_rom_file(
                path_ptr as u32,
                name.len() as u32,
                buf_ptr as u32,
                buf.len() as u32,
            );
        }
        File { raw: buf }
    }

    /// Read the whole file with the given name from ROM.
    ///
    /// If you have a pre-allocated buffer of the right size, use [load] instead.
    #[cfg(feature = "alloc")]
    pub fn load_buf(name: &str) -> FileBuf {
        let size = rom::get_size(name);
        let mut buf = vec![0; size];
        rom::load(name, &mut buf);
        FileBuf { raw: buf }
    }
}

/// Functions for accessing files in the app data dir.
pub mod data {
    use super::*;
    use crate::bindings as b;

    /// Get a file size in the data dir.
    ///
    /// If the file does not exist, 0 is returned.
    #[must_use]
    pub fn get_size(name: &str) -> usize {
        let path_ptr = name.as_ptr();
        let size = unsafe { b::get_file_size(path_ptr as u32, name.len() as u32) };
        size as usize
    }

    /// Read the whole file with the given name into the given buffer.
    ///
    /// If the file size is not known in advance (and so the buffer has to be allocated
    /// dynamically), consider using [load_buf] instead.
    pub fn load<'a>(name: &str, buf: &'a mut [u8]) -> File<'a> {
        let path_ptr = name.as_ptr();
        let buf_ptr = buf.as_mut_ptr();
        unsafe {
            b::load_file(
                path_ptr as u32,
                name.len() as u32,
                buf_ptr as u32,
                buf.len() as u32,
            );
        }
        File { raw: buf }
    }

    /// Read the whole file with the given name from the data dir.
    ///
    /// If you have a pre-allocated buffer of the right size, use [load] instead.
    #[cfg(feature = "alloc")]
    pub fn load_buf(name: &str) -> FileBuf {
        let size = data::get_size(name);
        let mut buf = vec![0; size];
        data::load(name, &mut buf);
        FileBuf { raw: buf }
    }

    /// Write the buffer into the given file in the data dir.
    ///
    /// If the file exists, it will be overwritten.
    /// If it doesn't exist, it will be created.
    pub fn dump(name: &str, buf: &[u8]) {
        let path_ptr = name.as_ptr();
        let buf_ptr = buf.as_ptr();
        unsafe {
            b::dump_file(
                path_ptr as u32,
                name.len() as u32,
                buf_ptr as u32,
                buf.len() as u32,
            );
        }
    }

    /// Remove file (if exists) with the given name from the data dir.
    pub fn remove(name: &str) {
        let path_ptr = name.as_ptr();
        unsafe {
            b::remove_file(path_ptr as u32, name.len() as u32);
        }
    }
}

/// A loaded font file.
///
/// Can be loaded as [FileBuf] from ROM with [rom::load_buf]
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

/// A loaded image file.
///
/// Can be loaded as [FileBuf] from ROM with [rom::load_buf]
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
    pub fn sub(&self, p: Point, s: Size) -> SubImage<'a> {
        SubImage {
            point: p,
            size:  s,
            raw:   self.raw,
        }
    }
}

/// A subregion of an image. Constructed using [Image::sub].
pub struct SubImage<'a> {
    pub(crate) point: Point,
    pub(crate) size:  Size,
    pub(crate) raw:   &'a [u8],
}
