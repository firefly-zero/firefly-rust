#![no_std]
#![no_main]
use firefly_rust as ff;

#[no_mangle]
extern fn boot() {
    ff::audio::OUT.add_sine(ff::audio::Freq::A4, 0.);
}
