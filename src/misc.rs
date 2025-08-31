use crate::net::Peer;

/// Log a debug message.
pub fn log_debug(t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        bindings::log_debug(ptr, len);
    }
}

/// Log an error message.
pub fn log_error(t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        bindings::log_error(ptr, len);
    }
}

/// Set the random seed.
pub fn set_seed(seed: u32) {
    unsafe {
        bindings::set_seed(seed);
    }
}

/// Get a random value.
#[must_use]
pub fn get_random() -> u32 {
    unsafe { bindings::get_random() }
}

/// Get the Peer's name.
#[cfg(feature = "alloc")]
#[must_use]
pub fn get_name_buf(p: Peer) -> alloc::string::String {
    let mut buf = [0u8; 16];
    let name = get_name(p, &mut buf);
    alloc::string::String::from(name)
}

#[must_use]
pub fn get_name(p: Peer, buf: &mut [u8; 16]) -> &str {
    let ptr = buf.as_ptr() as u32;
    let len = unsafe { bindings::get_name(u32::from(p.0), ptr) };
    let buf = &buf[..len as usize];
    unsafe { core::str::from_utf8_unchecked(buf) }
}

/// Exit the app after the current update is finished.
pub fn quit() {
    unsafe { bindings::quit() }
}

mod bindings {
    #[link(wasm_import_module = "misc")]
    extern "C" {
        pub(crate) fn log_debug(ptr: u32, len: u32);
        pub(crate) fn log_error(ptr: u32, len: u32);
        pub(crate) fn set_seed(seed: u32);
        pub(crate) fn get_random() -> u32;
        pub(crate) fn get_name(idx: u32, ptr: u32) -> u32;
        pub(crate) fn quit();
    }
}
