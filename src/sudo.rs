//! Functions available only to privileged apps.

use crate::fs::{File, FileBuf};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec;

#[cfg(feature = "alloc")]
pub struct DirBuf {
    raw: Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl DirBuf {
    /// List all subdirectories in the given directory.
    #[must_use]
    pub fn list_dirs(name: &str) -> Self {
        let size = Dir::list_dirs_buf_size(name);
        let mut buf = vec![0; size];
        Dir::list_dirs(name, &mut buf);
        Self {
            raw: buf.into_boxed_slice(),
        }
    }

    /// Iterate over all loaded entries in the directory.
    #[must_use]
    pub fn iter(&self) -> DirIter<'_> {
        DirIter { raw: &self.raw }
    }
}

pub struct Dir<'a> {
    raw: &'a [u8],
}

impl<'a> Dir<'a> {
    #[must_use]
    pub fn list_dirs_buf_size(name: &str) -> usize {
        let path_ptr = name.as_ptr();
        let size = unsafe { b::list_dirs_buf_size(path_ptr as u32, name.len() as u32) };
        size as usize
    }

    pub fn list_dirs(name: &str, buf: &'a mut [u8]) -> Self {
        let path_ptr = name.as_ptr();
        let buf_ptr = buf.as_mut_ptr();
        unsafe {
            b::list_dirs(
                path_ptr as u32,
                name.len() as u32,
                buf_ptr as u32,
                buf.len() as u32,
            );
        }
        Self { raw: buf }
    }

    /// Iterate over all loaded entries in the directory.
    #[must_use]
    pub const fn iter(&self) -> DirIter<'a> {
        DirIter { raw: self.raw }
    }
}

pub struct DirIter<'a> {
    raw: &'a [u8],
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.raw.first()?;
        let len = *len as usize;
        let raw_name = self.raw.get(1..=len)?;
        // Security: do NOT return None on invalid string. Otherwise,
        // adding an invalid name into the directory would hide all files
        // that go after that.
        let name = core::str::from_utf8(raw_name).unwrap_or("");
        self.raw = self.raw.get((len + 1)..).unwrap_or(&[]);
        Some(name)
    }
}

/// Tell runtime to exit the current app and replace it with the given one.
///
/// The exit will be executed after the current frame is rendered.
/// Calling this function does not interrup the current app execution.
pub fn run_app(author_id: &str, app_id: &str) {
    let author_ptr = author_id.as_ptr() as u32;
    let app_ptr = app_id.as_ptr() as u32;
    let author_len = author_id.len() as u32;
    let app_len = app_id.len() as u32;
    unsafe {
        b::run_app(author_ptr, author_len, app_ptr, app_len);
    }
}

#[must_use]
pub fn get_file_size(path: &str) -> usize {
    let path_ptr = path.as_ptr() as u32;
    let path_len = path.len() as u32;
    let size = unsafe { b::get_file_size(path_ptr, path_len) };
    size as usize
}

/// Read a file from the filesystem into the buffer.
///
/// The path must be relative to the root of the FS.
/// For example: `sys/launcher`.
pub fn load_file<'a>(path: &str, buf: &'a mut [u8]) -> File<'a> {
    let path_ptr = path.as_ptr() as u32;
    let path_len = path.len() as u32;
    let buf_ptr = buf.as_mut_ptr() as u32;
    let buf_len = buf.len() as u32;
    unsafe {
        b::load_file(path_ptr, path_len, buf_ptr, buf_len);
    }
    File { raw: buf }
}

/// Like [`load_file`] but takes care of the buffer allocation and ownership.
///
/// [`None`] is returned if the file does not exist.
#[cfg(feature = "alloc")]
#[must_use]
pub fn load_file_buf(path: &str) -> Option<FileBuf> {
    let size = get_file_size(path);
    if size == 0 {
        return None;
    }
    let mut buf = vec![0; size];
    let path_ptr = path.as_ptr() as u32;
    let path_len = path.len() as u32;
    let buf_ptr = buf.as_mut_ptr() as u32;
    let buf_len = buf.len() as u32;
    unsafe {
        b::load_file(path_ptr, path_len, buf_ptr, buf_len);
    }
    Some(FileBuf {
        raw: buf.into_boxed_slice(),
    })
}

/// Low-level bindings for host-defined "sudo" module.
mod b {
    #[link(wasm_import_module = "sudo")]
    extern "C" {
        pub(super) fn list_dirs_buf_size(path_ptr: u32, path_len: u32) -> u32;
        pub(super) fn list_dirs(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
        pub(super) fn run_app(author_ptr: u32, author_len: u32, app_ptr: u32, app_len: u32);
        pub(super) fn load_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
        pub(super) fn get_file_size(path_ptr: u32, path_len: u32) -> u32;
    }
}
