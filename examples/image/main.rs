//! The example shows how to read an image from ROM and draw it on the screen.
//!
//! Since we don't specify in advance the image size, we'll need an allocator
//! to dynamically allocate the required memory. A proper solution would be
//! to find a third-party allocator (like wee_alloc) and use that.
//! However, to keep the example simple, we instead enable std. It significantly
//! increases the binary size but it's good enough for simple apps.

#![allow(static_mut_refs)]
#![no_main]
#![no_std]
use core::cell::OnceCell;
use firefly_rust as ff;

static mut IMAGE: OnceCell<ff::ImageBuf> = OnceCell::new();

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let file = ff::load_file_buf("img").unwrap();
    unsafe {
        IMAGE.set(file.into()).ok().unwrap();
    }
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    ff::clear_screen(ff::Color::White);
    let image = unsafe { IMAGE.get().unwrap() };
    ff::draw_image(image, ff::Point { x: 60, y: 60 });
}
