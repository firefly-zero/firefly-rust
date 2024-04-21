use crate::graphics::{Point, Size};
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub struct FileBuf {
    raw: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl FileBuf {
    pub fn load(name: &str) -> Self {
        let size = rom::get_size(name);
        let mut buf = vec![0; size];
        rom::load(name, &mut buf);
        Self { raw: buf }
    }
}

pub struct File<'a> {
    raw: &'a [u8],
}

/// Functions for accessing files in the app ROM.
pub mod rom {
    use super::*;
    use crate::bindings as b;

    #[must_use]
    pub fn get_size(name: &str) -> usize {
        let path_ptr = name.as_ptr();
        let size = unsafe { b::get_rom_file_size(path_ptr as u32, name.len() as u32) };
        size as usize
    }

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
}

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
    pub fn sub(&self, p: Point, s: Size) -> SubImage<'a> {
        SubImage {
            point: p,
            size:  s,
            raw:   self.raw,
        }
    }
}

pub struct SubImage<'a> {
    pub(crate) point: Point,
    pub(crate) size:  Size,
    pub(crate) raw:   &'a [u8],
}
