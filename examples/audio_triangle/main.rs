#![no_std]
#![no_main]
use firefly_rust::audio;

#[no_mangle]
extern fn boot() {
    audio::OUT.add_triangle(audio::Freq::A4, 0.);
}
