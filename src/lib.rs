#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::pedantic)]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::allow_attributes)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![allow(clippy::wildcard_imports)]
#![expect(clippy::struct_excessive_bools)]
#![expect(clippy::cast_possible_truncation)]
#![expect(clippy::iter_without_into_iter)]

pub mod audio;
mod fs;
pub mod graphics;
mod input;
pub mod math;
mod menu;
mod misc;
mod net;
pub mod shapes;
#[cfg(feature = "sudo")]
pub mod sudo;

pub use fs::*;
pub use graphics::*;
pub use input::*;
pub use menu::*;
pub use misc::*;
pub use net::*;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(test), not(feature = "std"), target_family = "wasm"))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
