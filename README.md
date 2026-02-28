# firefly-rust

Rust SDK for making [Firefly Zero](https://fireflyzero.com/) games.

* [▶️ getting started](https://docs.fireflyzero.com/dev/getting-started/)
* [📄 api docs](https://docs.rs/firefly-rust/latest/firefly_rust/)
* [📦 crates.io](https://crates.io/crates/firefly-rust)
* [🐙 github](https://github.com/firefly-zero/firefly-rust)

## Installation

```bash
cargo add firefly_rust
```

## Cargo features

* `std`: required if you don't build your project with `#![no_std]`. It will remove from the crate the custom panic handler to avoid conflicts with the standard library.
* `alloc`: required if you want to use [`FileBuf`](https://docs.rs/firefly-rust/latest/firefly_rust/struct.FileBuf.html). Allows the crate to do memory allocations. If you enable `alloc` but not `std`, you have to also provide a global allocator. The easiest way to do so is to activate the `talc` feature (see below).
* `talc`: enable a global [talc](https://github.com/SFBdragon/talc)-powered allocator. The same as the `alloc` feature but you don't have to configure an allocator yourself.
* `panic-info`: if app panics, show panic info (message, file name, line number). Very useful for debugging but significantly increases the binary size.
* `sudo`: required if you want to use [`sudo`](https://docs.rs/firefly-rust/latest/firefly_rust/sudo/index.html) module. Enables privileged access to the Firefly Zero device.
* `firefly-toml`: use [firefly-toml](https://github.com/firefly-zero/firefly-toml) macro. Activating the feature will provide static access to boards, badges, cheats, and other data from `firefly.tom` via `firefly_rust::toml` module.
* `nalgebra`: can be activated if you use [nalgebra](https://github.com/dimforge/nalgebra). Enables type conversion to and from nalgebra vectors.

## License

MIT License. You can do whatever you want with the SDK, modify it, embed into any apps and games. Have fun!
