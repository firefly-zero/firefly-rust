#![no_std]
#![no_main]
use firefly_rust as ff;

#[no_mangle]
extern fn boot() {
    ff::audio::OUT.add_sine(ff::audio::Freq::C4, 0.);
    ff::audio::OUT.add_sine(ff::audio::Freq::E4, 0.);
    ff::audio::OUT.add_sine(ff::audio::Freq::G4, 0.);
}
