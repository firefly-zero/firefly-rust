#![no_std]
mod bindings;
mod graphics;

pub use graphics::*;

#[cfg(all(not(test), target_family = "wasm"))]
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
