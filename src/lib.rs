#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    clippy::pedantic,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::expect_used,
    clippy::unwrap_used,
    // missing_docs
)]
#![allow(clippy::wildcard_imports)]
#![expect(
    clippy::struct_excessive_bools,
    clippy::cast_possible_truncation,
    clippy::iter_without_into_iter
)]

pub mod audio;
mod fs;
pub mod graphics;
mod input;
pub mod math;
mod menu;
mod misc;
mod net;
pub mod shapes;
mod stats;
#[cfg(feature = "sudo")]
pub mod sudo;

pub use fs::*;
pub use graphics::*;
pub use input::*;
pub use menu::*;
pub use misc::*;
pub use net::*;
pub use stats::*;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(target_family = "wasm", feature = "talc"))]
#[global_allocator]
static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

#[cfg(all(not(test), not(feature = "std"), target_family = "wasm"))]
#[panic_handler]
#[allow(unused_variables)]
fn handle_panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(all(feature = "alloc", feature = "panic_info"))]
    if true {
        let msg = alloc::format!("{info}");
        log_error(&msg);
    }
    core::arch::wasm32::unreachable()
}

#[cfg(feature = "macros")]
pub use firefly_toml::import;
