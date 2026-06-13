#![no_std]
#![no_main]
use firefly_rust::audio;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let gain = audio::OUT.add_gain(0.);
    let modulator = audio::LinearModulator {
        start_at: audio::Time::ZERO,
        end_at: audio::seconds(2),
    };
    gain.modulate(0., 1., modulator);
    gain.add_sine(audio::Freq::A4, 0.);
}
