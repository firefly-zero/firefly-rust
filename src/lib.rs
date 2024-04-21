#![no_std]
mod bindings;
mod fs;
mod graphics;

pub use fs::*;
pub use graphics::*;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(test), not(feature = "std"), target_family = "wasm"))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
