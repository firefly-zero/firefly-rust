#![no_main]
use firefly_rust as ff;
use std::cell::OnceCell;

static mut IMAGE: OnceCell<ff::FileBuf> = OnceCell::new();

#[no_mangle]
extern fn boot() {
    let file = ff::FileBuf::load("img");
    unsafe {
        IMAGE.set(file).ok().unwrap();
    }
}

#[no_mangle]
extern fn update() {
    ff::clear_screen(ff::Color::LIGHT);
    let image = unsafe { IMAGE.get().unwrap() };
    let image: ff::Image = (image).into();
    let colors = ff::ImageColors::default();
    ff::draw_image(image, ff::Point { x: 60, y: 60 }, colors);
}
