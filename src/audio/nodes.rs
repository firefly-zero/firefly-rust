use core::marker::PhantomData;

use super::*;

/// A marker for a node created by [`Node::add_sine`].
pub struct Sine {}
/// A marker for a node created by [`Node::add_mix`].
pub struct Mix {}
/// A marker for a node created by [`Node::add_all_for_one`].
pub struct AllForOne {}
/// A marker for a node created by [`Node::add_gain`].
pub struct Gain {}
/// A marker for a node created by [`Node::add_loop`].
pub struct Loop {}
/// A marker for a node created by [`Node::add_concat`].
pub struct Concat {}
/// A marker for a node created by [`Node::add_pan`].
pub struct Pan {}
/// A marker for a node created by [`Node::add_mute`].
pub struct Mute {}
/// A marker for a node created by [`Node::add_pause`].
pub struct Pause {}
/// A marker for a node created by [`Node::add_track_position`].
pub struct TrackPosition {}
/// A marker for a node created by [`Node::add_low_pass`].
pub struct LowPass {}
/// A marker for a node created by [`Node::add_high_pass`].
pub struct HighPass {}
/// A marker for a node created by [`Node::add_take_left`].
pub struct TakeLeft {}
/// A marker for a node created by [`Node::add_take_right`].
pub struct TakeRight {}
/// A marker for a node created by [`Node::add_swap`].
pub struct Swap {}
/// A marker for a node created by [`Node::add_clip`].
pub struct Clip {}
/// A marker for a node created by [`Node::add_square`].
pub struct Square {}
/// A marker for a node created by [`Node::add_sawtooth`].
pub struct Sawtooth {}
/// A marker for a node created by [`Node::add_triangle`].
pub struct Triangle {}
/// A marker for a node created by [`Node::add_noise`].
pub struct Noise {}
/// A marker for a node created by [`Node::add_empty`].
pub struct Empty {}
/// A marker for a node created by [`Node::add_zero`].
pub struct Zero {}

/// A marker for a node created by [`Node::add_file`].
pub struct File {}

/// A marker for nodes that can have sub-nodes.
trait Parent {}
impl Parent for Mix {}
impl Parent for AllForOne {}
impl Parent for Gain {}
impl Parent for Loop {}
impl Parent for Concat {}
impl Parent for Pan {}
impl Parent for Mute {}
impl Parent for Pause {}
impl Parent for TrackPosition {}
impl Parent for LowPass {}
impl Parent for HighPass {}
impl Parent for TakeLeft {}
impl Parent for TakeRight {}
impl Parent for Swap {}
impl Parent for Clip {}

/// An audio node: a source, a sink, a filter, an effect, etc.
pub struct Node<F> {
    id: u32,
    /// A marker for a specific node type. Used to control which parameters can be modulated.
    _flavor: PhantomData<F>,
}

/// The output audio node. Mixes all inputs and plays them on the device's speaker.
pub const OUT: Node<Mix> = Node::new(0);

impl<F> Node<F> {
    #[must_use]
    const fn new(id: u32) -> Self {
        Self {
            id,
            _flavor: PhantomData,
        }
    }

    /// Reset the node state to how it was when it was just added.
    pub fn reset(&self) {
        unsafe { bindings::reset(self.id) }
    }
}

#[expect(clippy::must_use_candidate, private_bounds)]
impl<F: Parent> Node<F> {
    /// Add sine wave oscillator source (`∿`).
    pub fn add_sine(&self, f: Freq, phase: f32) -> Node<Sine> {
        let id = unsafe { bindings::add_sine(self.id, f.0, phase) };
        Node::new(id)
    }

    /// Add square wave oscillator source (`⎍`).
    pub fn add_square(&self, f: Freq, phase: f32) -> Node<Square> {
        let id = unsafe { bindings::add_square(self.id, f.0, phase) };
        Node::new(id)
    }

    /// Add sawtooth wave oscillator source (`╱│`).
    pub fn add_sawtooth(&self, f: Freq, phase: f32) -> Node<Sawtooth> {
        let id = unsafe { bindings::add_sawtooth(self.id, f.0, phase) };
        Node::new(id)
    }

    /// Add triangle wave oscillator source (`╱╲`).
    pub fn add_triangle(&self, f: Freq, phase: f32) -> Node<Triangle> {
        let id = unsafe { bindings::add_triangle(self.id, f.0, phase) };
        Node::new(id)
    }

    /// Add white noise source (amplitude on each tick is random).
    pub fn add_noise(&self, seed: i32) -> Node<Noise> {
        let id = unsafe { bindings::add_noise(self.id, seed) };
        Node::new(id)
    }

    /// Add always stopped source.
    pub fn add_empty(&self) -> Node<Empty> {
        let id = unsafe { bindings::add_empty(self.id) };
        Node::new(id)
    }

    /// Add silent source producing zeros.
    pub fn add_zero(&self) -> Node<Zero> {
        let id = unsafe { bindings::add_zero(self.id) };
        Node::new(id)
    }

    /// Play an audio file from ROM.
    pub fn add_file(&self, path: &str) -> Node<File> {
        let ptr = path.as_ptr() as u32;
        let len = path.len() as u32;
        let id = unsafe { bindings::add_file(self.id, ptr, len) };
        Node::new(id)
    }

    /// Add node simply mixing all inputs.
    pub fn add_mix(&self) -> Node<Mix> {
        let id = unsafe { bindings::add_mix(self.id) };
        Node::new(id)
    }

    /// Add mixer node that stops if any of the sources stops.
    pub fn add_all_for_one(&self) -> Node<AllForOne> {
        let id = unsafe { bindings::add_all_for_one(self.id) };
        Node::new(id)
    }

    /// Add gain control node.
    pub fn add_gain(&self, lvl: f32) -> Node<Gain> {
        let id = unsafe { bindings::add_gain(self.id, lvl) };
        Node::new(id)
    }

    /// Add a loop node that resets the input if it stops.
    pub fn add_loop(&self) -> Node<Loop> {
        let id = unsafe { bindings::add_loop(self.id) };
        Node::new(id)
    }

    /// Add a node that plays the inputs one after the other, in the order as they added.
    pub fn add_concat(&self) -> Node<Concat> {
        let id = unsafe { bindings::add_concat(self.id) };
        Node::new(id)
    }

    /// Add node panning the audio to the left (0.), right (1.), or something in between.
    pub fn add_pan(&self, lvl: f32) -> Node<Pan> {
        let id = unsafe { bindings::add_pan(self.id, lvl) };
        Node::new(id)
    }

    /// Add node that can be muted using modulation.
    pub fn add_mute(&self) -> Node<Mute> {
        let id = unsafe { bindings::add_mute(self.id) };
        Node::new(id)
    }

    /// Add node that can be paused using modulation.
    pub fn add_pause(&self) -> Node<Pause> {
        let id = unsafe { bindings::add_pause(self.id) };
        Node::new(id)
    }

    /// Add node tracking the elapsed playback time.
    pub fn add_track_position(&self) -> Node<TrackPosition> {
        let id = unsafe { bindings::add_track_position(self.id) };
        Node::new(id)
    }

    /// Add lowpass filter node.
    pub fn add_low_pass(&self, freq: f32, q: f32) -> Node<LowPass> {
        let id = unsafe { bindings::add_low_pass(self.id, freq, q) };
        Node::new(id)
    }

    /// Add highpass filter node.
    pub fn add_high_pass(&self, freq: f32, q: f32) -> Node<HighPass> {
        let id = unsafe { bindings::add_high_pass(self.id, freq, q) };
        Node::new(id)
    }

    /// Add node converting stereo to mono by taking the left channel.
    pub fn add_take_left(&self) -> Node<TakeLeft> {
        let id = unsafe { bindings::add_take_left(self.id) };
        Node::new(id)
    }

    /// Add node converting stereo to mono by taking the right channel.
    pub fn add_take_right(&self) -> Node<TakeRight> {
        let id = unsafe { bindings::add_take_right(self.id) };
        Node::new(id)
    }

    /// Add node swapping left and right channels of the stereo input.
    pub fn add_swap(&self) -> Node<Swap> {
        let id = unsafe { bindings::add_swap(self.id) };
        Node::new(id)
    }

    /// Add node clamping the input amplitude. Can be used for hard distortion.
    pub fn add_clip(&self, low: f32, high: f32) -> Node<Clip> {
        let id = unsafe { bindings::add_clip(self.id, low, high) };
        Node::new(id)
    }

    /// Reset the node and all child nodes to the state to how it was when they were just added.
    pub fn reset_all(&self) {
        unsafe { bindings::reset_all(self.id) }
    }

    /// Remove all child nodes.
    ///
    /// After it is called, you should make sure to discard all references to the old
    /// child nodes.
    pub fn clear(&self) {
        unsafe { bindings::clear(self.id) }
    }
}

impl Node<Sine> {
    /// Modulate oscillation frequency.
    pub fn modulate<M: Modulator>(&self, low: Freq, high: Freq, m: M) {
        m.modulate(self.id, 0, low.0, high.0);
    }

    /// Set the oscillation frequency.
    pub fn set(&self, val: Freq) {
        unsafe { bindings::set_param(self.id, 0, val.0) };
    }
}

impl Node<Square> {
    /// Modulate oscillation frequency.
    pub fn modulate<M: Modulator>(&self, low: Freq, high: Freq, m: M) {
        m.modulate(self.id, 0, low.0, high.0);
    }

    /// Set the oscillation frequency.
    pub fn set(&self, val: Freq) {
        unsafe { bindings::set_param(self.id, 0, val.0) };
    }
}

impl Node<Sawtooth> {
    /// Modulate oscillation frequency.
    pub fn modulate<M: Modulator>(&self, low: Freq, high: Freq, m: M) {
        m.modulate(self.id, 0, low.0, high.0);
    }

    /// Set the oscillation frequency.
    pub fn set(&self, val: Freq) {
        unsafe { bindings::set_param(self.id, 0, val.0) };
    }
}

impl Node<Triangle> {
    /// Modulate oscillation frequency.
    pub fn modulate<M: Modulator>(&self, low: Freq, high: Freq, m: M) {
        m.modulate(self.id, 0, low.0, high.0);
    }

    /// Set the oscillation frequency.
    pub fn set(&self, val: Freq) {
        unsafe { bindings::set_param(self.id, 0, val.0) };
    }
}

impl Node<File> {
    /// Go to the specified timestamp in the file.
    #[expect(clippy::cast_precision_loss)]
    pub fn seek(&self, t: Time) {
        unsafe { bindings::set_param(self.id, 0, t.0 as f32) };
    }
}

impl Node<Gain> {
    /// Modulate the gain level.
    pub fn modulate<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 0, low, high);
    }

    /// Set the the gain level.
    pub fn set(&self, val: f32) {
        unsafe { bindings::set_param(self.id, 0, val) };
    }
}

impl Node<Pan> {
    /// Modulate the pan value (from 0. to 1.: 0. is only left, 1. is only right).
    pub fn modulate<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 0, low, high);
    }

    /// Set the the pan value (from 0. to 1.: 0. is only left, 1. is only right).
    pub fn set(&self, val: f32) {
        unsafe { bindings::set_param(self.id, 0, val) };
    }
}

impl Node<Mute> {
    /// Modulate the muted state.
    ///
    /// Below 0.5 is muted, above is unmuted.
    pub fn modulate<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 0, low, high);
    }

    pub fn mute(&self) {
        unsafe { bindings::set_param(self.id, 0, 0.) };
    }

    pub fn unmute(&self) {
        unsafe { bindings::set_param(self.id, 0, 1.) };
    }
}

impl Node<Pause> {
    /// Modulate the paused state.
    ///
    /// Below 0.5 is paused, above is playing.
    pub fn modulate<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 0, low, high);
    }

    pub fn pause(&self) {
        unsafe { bindings::set_param(self.id, 0, 0.) };
    }

    pub fn play(&self) {
        unsafe { bindings::set_param(self.id, 0, 1.) };
    }
}

impl Node<LowPass> {
    /// Modulate the cut-off frequency.
    pub fn modulate_freq<M: Modulator>(&self, low: Freq, high: Freq, m: M) {
        m.modulate(self.id, 0, low.0, high.0);
    }

    /// Set the the cut-off frequency.
    pub fn set_freq(&self, val: Freq) {
        unsafe { bindings::set_param(self.id, 0, val.0) };
    }
}

impl Node<HighPass> {
    /// Modulate the cut-off frequency.
    pub fn modulate_freq<M: Modulator>(&self, low: Freq, high: Freq, m: M) {
        m.modulate(self.id, 0, low.0, high.0);
    }

    /// Set the the cut-off frequency.
    pub fn set_freq(&self, val: Freq) {
        unsafe { bindings::set_param(self.id, 0, val.0) };
    }
}

impl Node<Clip> {
    /// Modulate the low cut amplitude and adjust the high amplitude to keep the gap.
    ///
    /// In other words, the difference between low and high cut points will stay the same.
    pub fn modulate_both<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 0, low, high);
    }

    /// Set the the low cut amplitude and adjust the high amplitude to keep the gap.
    pub fn set_both(&self, val: f32) {
        unsafe { bindings::set_param(self.id, 0, val) };
    }

    /// Modulate the low cut amplitude.
    pub fn modulate_low<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 1, low, high);
    }

    /// Set the the low cut amplitude.
    pub fn set_low(&self, val: f32) {
        unsafe { bindings::set_param(self.id, 1, val) };
    }

    /// Modulate the high cut amplitude.
    pub fn modulate_high<M: Modulator>(&self, low: f32, high: f32, m: M) {
        m.modulate(self.id, 2, low, high);
    }

    /// Set the the high cut amplitude.
    pub fn set_high(&self, val: f32) {
        unsafe { bindings::set_param(self.id, 2, val) };
    }
}

mod bindings {
    #[link(wasm_import_module = "audio")]
    unsafe extern "C" {
        // generators
        pub(super) unsafe fn add_sine(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) unsafe fn add_square(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) unsafe fn add_sawtooth(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) unsafe fn add_triangle(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) unsafe fn add_noise(parent_id: u32, seed: i32) -> u32;
        pub(super) unsafe fn add_empty(parent_id: u32) -> u32;
        pub(super) unsafe fn add_zero(parent_id: u32) -> u32;
        pub(super) unsafe fn add_file(parent: u32, ptr: u32, len: u32) -> u32;

        // nodes
        pub(super) unsafe fn add_mix(parent_id: u32) -> u32;
        pub(super) unsafe fn add_all_for_one(parent_id: u32) -> u32;
        pub(super) unsafe fn add_gain(parent_id: u32, lvl: f32) -> u32;
        pub(super) unsafe fn add_loop(parent_id: u32) -> u32;
        pub(super) unsafe fn add_concat(parent_id: u32) -> u32;
        pub(super) unsafe fn add_pan(parent_id: u32, lvl: f32) -> u32;
        pub(super) unsafe fn add_mute(parent_id: u32) -> u32;
        pub(super) unsafe fn add_pause(parent_id: u32) -> u32;
        pub(super) unsafe fn add_track_position(parent_id: u32) -> u32;
        pub(super) unsafe fn add_low_pass(parent_id: u32, freq: f32, q: f32) -> u32;
        pub(super) unsafe fn add_high_pass(parent_id: u32, freq: f32, q: f32) -> u32;
        pub(super) unsafe fn add_take_left(parent_id: u32) -> u32;
        pub(super) unsafe fn add_take_right(parent_id: u32) -> u32;
        pub(super) unsafe fn add_swap(parent_id: u32) -> u32;
        pub(super) unsafe fn add_clip(parent_id: u32, low: f32, high: f32) -> u32;

        pub(super) unsafe fn reset(node_id: u32);
        pub(super) unsafe fn reset_all(node_id: u32);
        pub(super) unsafe fn clear(node_id: u32);
        pub(super) unsafe fn set_param(node_id: u32, param: u32, val: f32);
    }
}
