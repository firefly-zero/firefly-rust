//! The example shows how to read an image from ROM and draw it on the screen.
//!
//! Since we don't specify in advance the image size, we'll need an allocator
//! to dynamically allocate the required memory. A proper solution would be
//! to find a third-party allocator (like wee_alloc) and use that.
//! However, to keep the example simple, we instead enable std. It significantly
//! increases the binary size but it's good enough for simple apps.

#![allow(static_mut_refs)]
#![no_main]
use firefly_rust as ff;
use std::cell::OnceCell;

static mut IMAGE: OnceCell<ff::FileBuf> = OnceCell::new();

#[no_mangle]
extern "C" fn boot() {
    let file = ff::load_file_buf("img").unwrap();
    unsafe {
        IMAGE.set(file).ok().unwrap();
    }
}

#[no_mangle]
extern "C" fn update() {
    ff::clear_screen(ff::Color::White);
    let image = unsafe { IMAGE.get().unwrap() };
    let image: ff::Image = (image).into();
    ff::draw_image(&image, ff::Point { x: 60, y: 60 });
}
