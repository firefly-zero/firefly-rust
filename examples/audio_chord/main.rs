#![no_std]
#![no_main]
use firefly_rust::audio;

#[no_mangle]
extern "C" fn boot() {
    audio::OUT.add_sine(audio::Freq::C4, 0.);
    audio::OUT.add_sine(audio::Freq::E4, 0.);
    audio::OUT.add_sine(audio::Freq::G4, 0.);
}
