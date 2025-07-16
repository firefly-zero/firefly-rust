use super::*;

pub trait Modulator {
    fn modulate(self, node_id: u32, param: u32);
}

/// Linear (ramp up or down) envelope.
///
/// It looks like this: `⎽╱⎺` or `⎺╲⎽`.
///
/// The value before `start_at` is `start`, the value after `end_at` is `end`,
/// and the value between `start_at` and `end_at` changes linearly from `start` to `end`.
pub struct LinearModulator {
    pub start: f32,
    pub end: f32,
    pub start_at: Time,
    pub end_at: Time,
}

impl Modulator for LinearModulator {
    fn modulate(self, node_id: u32, param: u32) {
        unsafe {
            bindings::mod_linear(
                node_id,
                param,
                self.start,
                self.end,
                self.start_at.0,
                self.end_at.0,
            );
        }
    }
}

/// Hold envelope.
///
/// It looks like this: `⎽│⎺` or `⎺│⎽`.
///
/// The value before `time` is `before` and the value after `time` is `after`.
/// Equivalent to [`LinearModulator`] with `start_at` being equal to `end_at`.
pub struct HoldModulator {
    pub before: f32,
    pub after: f32,
    pub time: Time,
}

impl Modulator for HoldModulator {
    fn modulate(self, node_id: u32, param: u32) {
        unsafe {
            bindings::mod_hold(node_id, param, self.before, self.after, self.time.0);
        }
    }
}

/// Sine wave low-frequency oscillator.
///
/// It looks like this: `∿`.
///
/// `low` is the lowest produced value, `high` is the highest.
pub struct SineModulator {
    pub freq: Freq,
    pub low: f32,
    pub high: f32,
}

impl Modulator for SineModulator {
    fn modulate(self, node_id: u32, param: u32) {
        unsafe {
            bindings::mod_sine(node_id, param, self.freq.0, self.low, self.high);
        }
    }
}

mod bindings {
    #[link(wasm_import_module = "audio")]
    extern "C" {
        pub(super) fn mod_linear(
            node_id: u32,
            param: u32,
            start: f32,
            end: f32,
            start_at: u32,
            end_at: u32,
        );
        pub(super) fn mod_hold(node_id: u32, param: u32, v1: f32, v2: f32, time: u32);
        pub(super) fn mod_sine(node_id: u32, param: u32, freq: f32, low: f32, high: f32);
    }
}
