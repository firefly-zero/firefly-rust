use crate::bindings::sudo as b;
use crate::fs::{File, FileBuf};
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub struct DirBuf {
    raw: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl DirBuf {
    pub fn list_dirs(name: &str) -> Self {
        let size = Dir::list_dirs_buf_size(name);
        let mut buf = vec![0; size];
        Dir::list_dirs(name, &mut buf);
        Self { raw: buf }
    }

    pub fn iter(&self) -> DirIter<'_> {
        DirIter { raw: &self.raw }
    }
}

pub struct Dir<'a> {
    raw: &'a [u8],
}

impl<'a> Dir<'a> {
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

    pub fn iter(&self) -> DirIter<'a> {
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
        let raw_name = self.raw.get(1..(len + 1))?;
        // Security: do NOT return None on invalid string. Otherwise,
        // adding an invalid name into the directory would hide all files
        // that go after that.
        let name = core::str::from_utf8(raw_name).unwrap_or("");
        self.raw = self.raw.get((len + 1)..).unwrap_or(&[]);
        Some(name)
    }
}

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

#[cfg(feature = "alloc")]
pub fn load_file_buf(path: &str) -> FileBuf {
    let size = get_file_size(path);
    let mut buf = vec![0; size];
    let path_ptr = path.as_ptr() as u32;
    let path_len = path.len() as u32;
    let buf_ptr = buf.as_mut_ptr() as u32;
    let buf_len = buf.len() as u32;
    unsafe {
        b::load_file(path_ptr, path_len, buf_ptr, buf_len);
    }
    FileBuf { raw: buf }
}
