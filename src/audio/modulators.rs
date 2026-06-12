use super::*;

/// Modulator can be attached to a node to change a node parameter over time.
///
/// Modulators include both LFOs (Low-Frequency Oscillator) and envelopes.
/// The difference is that LFOs keep oscillating between values
/// while envelopes go from one value to another and then stop.
///
/// Internally, modulators only produce values from 0 to 1.
/// Then, to get the final value of the parameter,
/// the value from the modulator is projected on the range
/// between `low` and `high` arguments passed together
/// with the modulator when attaching a modulator to a node.
/// For example, [`Node<Sine>::modulate`] accepts the range of modulated values
/// for the sine wave frequency (which can be used for vibrato effect).
///
/// Even if a node has multiple parameters that can be modulated,
/// currently  single node may have at most one modulator attached.
pub trait Modulator {
    fn modulate(self, node_id: u32, param: u32, low: f32, high: f32);
}

/// Linear (ramp up or down) envelope.
///
/// It looks like this: `⎽╱⎺` (or `⎺╲⎽` if `low` is bigger than `high`).
///
/// The value before `start_at` is 0, the value after `end_at` is 1,
/// and the value between `start_at` and `end_at` changes linearly from 0 to 1.
///
/// Most often used with [`Gain`] for fade in and fade out effect.
pub struct LinearModulator {
    pub start_at: Time,
    pub end_at: Time,
}

impl Modulator for LinearModulator {
    fn modulate(self, node_id: u32, param: u32, low: f32, high: f32) {
        unsafe {
            bindings::mod_linear(node_id, param, low, high, self.start_at.0, self.end_at.0);
        }
    }
}

/// Hold envelope.
///
/// It looks like this: `⎽│⎺` (or `⎺│⎽` if `low` is bigger than `high`).
///
/// The value before `time` is 0 and the value after `time` is 1.
/// Equivalent to [`LinearModulator`] with `start_at` being equal to `end_at`.
pub struct HoldModulator {
    pub time: Time,
}

impl Modulator for HoldModulator {
    fn modulate(self, node_id: u32, param: u32, low: f32, high: f32) {
        unsafe {
            bindings::mod_hold(node_id, param, low, high, self.time.0);
        }
    }
}

/// ADSR envelope.
///
/// It looks like this: `🭋🭍🬹🬿`
///
///  1. Until `attack`, the value goes from 0 to 1;
///  2. Until `decay`, it goes from 1 to `sustain_level`;
///  3. Until `sustain`, it holds `sustain_level`;
///  4. Until `release`, it goes from `sustain_level` to 0;
///  5. After `release`, it holds 0.
///
/// Most commonly used with [`Gain`].
pub struct AdsrModulator {
    /// When the value reaches 1.
    attack: Time,
    /// When the value reaches `sustain_level`.
    decay: Time,
    /// Until when the value holds `sustain_level`.
    sustain: Time,
    /// The value generated from `decay` until `sustain`.
    sustain_level: f32,
    /// When the value drops to 0.
    release: Time,
}

impl Modulator for AdsrModulator {
    fn modulate(self, node_id: u32, param: u32, low: f32, high: f32) {
        unsafe {
            bindings::mod_adsr(
                node_id,
                param,
                low,
                high,
                self.attack.0,
                self.decay.0,
                self.sustain.0,
                self.sustain_level,
                self.release.0,
            );
        }
    }
}

/// Sine wave low-frequency oscillator.
///
/// It looks like this: `∿`.
///
/// Most commonly used with [`Sine`] (or another wave generator)
/// to produce vibrato effect.
pub struct SineModulator {
    pub freq: Freq,
}

impl Modulator for SineModulator {
    fn modulate(self, node_id: u32, param: u32, low: f32, high: f32) {
        unsafe {
            bindings::mod_sine(node_id, param, self.freq.0, low, high);
        }
    }
}

mod bindings {
    #[link(wasm_import_module = "audio")]
    unsafe extern "C" {
        pub(super) unsafe fn mod_linear(
            node_id: u32,
            param: u32,
            start: f32,
            end: f32,
            start_at: u32,
            end_at: u32,
        );
        pub(super) unsafe fn mod_hold(node_id: u32, param: u32, v1: f32, v2: f32, time: u32);
        pub(super) unsafe fn mod_sine(node_id: u32, param: u32, freq: f32, low: f32, high: f32);
        pub(super) unsafe fn mod_adsr(
            node_id: u32,
            param: u32,
            low: f32,
            high: f32,
            attack: u32,
            decay: u32,
            sustain: u32,
            sustain_level: f32,
            release: u32,
        );
    }
}
