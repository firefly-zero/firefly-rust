//! Access file system: the game ROM files and the data dir.

use crate::*;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec;

/// Like [File] but owns the buffer.
///
/// Returned by [`load_file_buf`]. Requires a global allocator.
/// For a file of statically-known size, you might want to use [`load_file`] instead.
#[cfg(feature = "alloc")]
pub struct FileBuf {
    pub(crate) raw: Box<[u8]>,
}

#[cfg(feature = "alloc")]
impl FileBuf {
    /// Construct [`FileBuf`] from raw bytes.
    ///
    /// The main purpose of this function is to support convering [`FileBuf`]
    /// to and from "basic" types, which might be required for implementing
    /// some language interpreters (Lua, Python, etc) for Firefly in Rust.
    ///
    /// ## Safety
    ///
    /// This function allows bypassing type safety and constructing [`Image`]
    /// and [`Font`] in runtime. Don't do that. Relying on internal representation
    /// of file formats might make your app incompatible with future Firefly runtimes.
    /// If you need to modify an in-memory image, use [`Canvas`] instead.
    #[must_use]
    pub unsafe fn from_bytes(b: Box<[u8]>) -> Self {
        Self { raw: b }
    }

    #[must_use]
    pub fn into_font(self) -> FontBuf {
        self.into()
    }

    #[must_use]
    pub fn into_image(self) -> ImageBuf {
        self.into()
    }

    #[must_use]
    pub fn into_bytes(self) -> Box<[u8]> {
        self.into()
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
        value.raw.into_vec()
    }
}

#[cfg(feature = "alloc")]
impl<'a> From<&'a FileBuf> for &'a [u8] {
    fn from(value: &'a FileBuf) -> Self {
        &value.raw
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<FileBuf> for alloc::string::String {
    type Error = alloc::string::FromUtf8Error;

    fn try_from(value: FileBuf) -> Result<Self, Self::Error> {
        let v = value.raw.into_vec();
        alloc::string::String::from_utf8(v)
    }
}

/// A file loaded from ROM or data dir into the memory.
///
/// Returned by [`rom::load`] and [`data::load`] which requires a pre-allocated buffer
/// of the right size. If the file size is deterimed dynamically,
/// you might want to use [`rom::load_buf`] and [`data::load_buf`] instead
/// (which will take care of the dynamic allocation).
pub struct FileRef<'a> {
    pub(crate) raw: &'a [u8],
}

impl<'a> FileRef<'a> {
    /// Construct [`File`] from raw bytes.
    ///
    /// The main purpose of this function is to support convering [`File`]
    /// to and from "basic" types, which might be required for implementing
    /// some language interpreters (Lua, Python, etc) for Firefly in Rust.
    ///
    /// ## Safety
    ///
    /// This function allows bypassing type safety and constructing [`Image`]
    /// and [`Font`] in runtime. Don't do that. Relying on internal representation
    /// of file formats might make your app incompatible with future Firefly runtimes.
    /// If you need to modify an in-memory image, use [`Canvas`] instead.
    #[must_use]
    pub unsafe fn from_bytes(b: &'a [u8]) -> Self {
        Self { raw: b }
    }

    #[must_use]
    pub fn into_font(self) -> FontRef<'a> {
        self.into()
    }

    #[must_use]
    pub fn into_image(self) -> ImageRef<'a> {
        self.into()
    }

    #[must_use]
    pub fn into_bytes(self) -> &'a [u8] {
        self.into()
    }
}

impl<'a> From<FileRef<'a>> for &'a [u8] {
    fn from(value: FileRef<'a>) -> Self {
        value.raw
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
pub fn load_file<'a>(name: &str, buf: &'a mut [u8]) -> FileRef<'a> {
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
    FileRef { raw: buf }
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

mod bindings {
    #[link(wasm_import_module = "fs")]
    unsafe extern "C" {
        pub(crate) unsafe fn get_file_size(path_ptr: u32, path_len: u32) -> u32;
        pub(crate) unsafe fn load_file(
            path_ptr: u32,
            path_len: u32,
            buf_ptr: u32,
            buf_len: u32,
        ) -> u32;
        pub(crate) unsafe fn dump_file(
            path_ptr: u32,
            path_len: u32,
            buf_ptr: u32,
            buf_len: u32,
        ) -> u32;
        pub(crate) unsafe fn remove_file(path_ptr: u32, path_len: u32);
    }
}
