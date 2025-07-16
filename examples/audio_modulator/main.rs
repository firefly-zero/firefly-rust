#![no_std]
#![no_main]
use firefly_rust::audio;

#[no_mangle]
extern "C" fn boot() {
    let gain = audio::OUT.add_gain(0.);
    gain.modulate(audio::LinearModulator {
        start: 0.,
        end: 1.,
        start_at: audio::Time::ZERO,
        end_at: audio::Time::seconds(2),
    });
    gain.add_sine(audio::Freq::A4, 0.);
}
