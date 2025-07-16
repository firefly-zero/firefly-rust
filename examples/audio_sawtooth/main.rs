#![no_std]
#![no_main]
use firefly_rust::audio;

#[no_mangle]
extern "C" fn boot() {
    audio::OUT.add_sawtooth(audio::Freq::A4, 0.);
}
