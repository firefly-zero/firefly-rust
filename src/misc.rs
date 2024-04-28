use crate::bindings as b;

/// Log a debug message.
pub fn log_debug(t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        b::log_debug(ptr, len);
    }
}

/// Log an error message.
pub fn log_error(t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        b::log_error(ptr, len);
    }
}

/// Set the random seed.
pub fn set_seed(seed: u32) {
    unsafe {
        b::set_seed(seed);
    }
}

/// Get a random value.
pub fn get_random() -> u32 {
    unsafe { b::get_random() }
}
