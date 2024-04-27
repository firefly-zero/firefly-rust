#![no_std]
mod bindings;
mod fs;
mod graphics;
mod input;
pub mod math;
mod net;
#[cfg(feature = "sudo")]
pub mod sudo;

pub use fs::*;
pub use graphics::*;
pub use input::*;
pub use net::*;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(test), not(feature = "std"), target_family = "wasm"))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
