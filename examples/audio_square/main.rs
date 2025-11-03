#![no_std]
#![no_main]
use firefly_rust::audio;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    audio::OUT.add_square(audio::Freq::A4, 0.);
}
