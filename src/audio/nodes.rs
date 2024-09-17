use core::marker::PhantomData;

use super::*;

pub struct Root {}
pub struct Sine {}
pub struct Mix {}
pub struct AllForOne {}
pub struct Gain {}
pub struct Loop {}
pub struct Concat {}
pub struct Pan {}
pub struct Mute {}
pub struct Pause {}
pub struct TrackPosition {}
pub struct LowPass {}
pub struct HighPass {}
pub struct TakeLeft {}
pub struct TakeRight {}
pub struct Swap {}
pub struct Clip {}
pub struct Square {}
pub struct Sawtooth {}
pub struct Triangle {}
pub struct Noise {}
pub struct Empty {}
pub struct Zero {}

pub struct Node<F> {
    id: u32,
    _flavor: PhantomData<F>,
}

pub const OUT: Node<Root> = Node::new(0);

#[expect(clippy::must_use_candidate)]
impl<F> Node<F> {
    const fn new(id: u32) -> Self {
        Self {
            id,
            _flavor: PhantomData,
        }
    }

    pub fn add_sine(&self, f: Freq, phase: f32) -> Node<Sine> {
        let id = unsafe { bindings::add_sine(self.id, f.0, phase) };
        Node::new(id)
    }

    pub fn add_square(&self, f: Freq, phase: f32) -> Node<Square> {
        let id = unsafe { bindings::add_square(self.id, f.0, phase) };
        Node::new(id)
    }

    pub fn add_sawtooth(&self, f: Freq, phase: f32) -> Node<Sawtooth> {
        let id = unsafe { bindings::add_sawtooth(self.id, f.0, phase) };
        Node::new(id)
    }

    pub fn add_triangle(&self, f: Freq, phase: f32) -> Node<Triangle> {
        let id = unsafe { bindings::add_triangle(self.id, f.0, phase) };
        Node::new(id)
    }

    pub fn add_noise(&self, seed: i32) -> Node<Noise> {
        let id = unsafe { bindings::add_noise(self.id, seed) };
        Node::new(id)
    }

    pub fn add_empty(&self) -> Node<Empty> {
        let id = unsafe { bindings::add_empty(self.id) };
        Node::new(id)
    }

    pub fn add_zero(&self) -> Node<Zero> {
        let id = unsafe { bindings::add_zero(self.id) };
        Node::new(id)
    }

    pub fn add_mix(&self) -> Node<Mix> {
        let id = unsafe { bindings::add_mix(self.id) };
        Node::new(id)
    }

    pub fn add_all_for_one(&self) -> Node<AllForOne> {
        let id = unsafe { bindings::add_all_for_one(self.id) };
        Node::new(id)
    }

    pub fn add_gain(&self, lvl: f32) -> Node<Gain> {
        let id = unsafe { bindings::add_gain(self.id, lvl) };
        Node::new(id)
    }

    pub fn add_loop(&self) -> Node<Loop> {
        let id = unsafe { bindings::add_loop(self.id) };
        Node::new(id)
    }

    pub fn add_concat(&self) -> Node<Concat> {
        let id = unsafe { bindings::add_concat(self.id) };
        Node::new(id)
    }

    pub fn add_pan(&self, lvl: f32) -> Node<Pan> {
        let id = unsafe { bindings::add_pan(self.id, lvl) };
        Node::new(id)
    }

    pub fn add_mute(&self) -> Node<Mute> {
        let id = unsafe { bindings::add_mute(self.id) };
        Node::new(id)
    }

    pub fn add_pause(&self) -> Node<Pause> {
        let id = unsafe { bindings::add_pause(self.id) };
        Node::new(id)
    }

    pub fn add_track_position(&self) -> Node<TrackPosition> {
        let id = unsafe { bindings::add_track_position(self.id) };
        Node::new(id)
    }

    pub fn add_low_pass(&self, freq: f32, q: f32) -> Node<LowPass> {
        let id = unsafe { bindings::add_low_pass(self.id, freq, q) };
        Node::new(id)
    }

    pub fn add_high_pass(&self, freq: f32, q: f32) -> Node<HighPass> {
        let id = unsafe { bindings::add_high_pass(self.id, freq, q) };
        Node::new(id)
    }

    pub fn add_take_left(&self) -> Node<TakeLeft> {
        let id = unsafe { bindings::add_take_left(self.id) };
        Node::new(id)
    }

    pub fn add_take_right(&self) -> Node<TakeRight> {
        let id = unsafe { bindings::add_take_right(self.id) };
        Node::new(id)
    }

    pub fn add_swap(&self) -> Node<Swap> {
        let id = unsafe { bindings::add_swap(self.id) };
        Node::new(id)
    }

    pub fn add_clip(&self, low: f32, high: f32) -> Node<Clip> {
        let id = unsafe { bindings::add_clip(self.id, low, high) };
        Node::new(id)
    }

    pub fn reset(&self) {
        unsafe { bindings::reset(self.id) }
    }

    pub fn reset_all(&self) {
        unsafe { bindings::reset_all(self.id) }
    }

    pub fn clear(&self) {
        unsafe { bindings::clear(self.id) }
    }
}

impl Node<Gain> {
    pub fn modulate<M: Modulator>(&self, m: M) {
        m.modulate(self.id, 0);
    }
}

impl Node<Pan> {
    pub fn modulate<M: Modulator>(&self, m: M) {
        m.modulate(self.id, 0);
    }
}

mod bindings {
    #[link(wasm_import_module = "audio")]
    extern {
        pub(super) fn add_sine(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) fn add_square(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) fn add_sawtooth(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) fn add_triangle(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) fn add_noise(parent_id: u32, seed: i32) -> u32;
        pub(super) fn add_empty(parent_id: u32) -> u32;
        pub(super) fn add_zero(parent_id: u32) -> u32;

        pub(crate) fn add_mix(parent_id: u32) -> u32;
        pub(crate) fn add_all_for_one(parent_id: u32) -> u32;
        pub(crate) fn add_gain(parent_id: u32, lvl: f32) -> u32;
        pub(crate) fn add_loop(parent_id: u32) -> u32;
        pub(crate) fn add_concat(parent_id: u32) -> u32;
        pub(crate) fn add_pan(parent_id: u32, lvl: f32) -> u32;
        pub(crate) fn add_mute(parent_id: u32) -> u32;
        pub(crate) fn add_pause(parent_id: u32) -> u32;
        pub(crate) fn add_track_position(parent_id: u32) -> u32;
        pub(crate) fn add_low_pass(parent_id: u32, freq: f32, q: f32) -> u32;
        pub(crate) fn add_high_pass(parent_id: u32, freq: f32, q: f32) -> u32;
        pub(crate) fn add_take_left(parent_id: u32) -> u32;
        pub(crate) fn add_take_right(parent_id: u32) -> u32;
        pub(crate) fn add_swap(parent_id: u32) -> u32;
        pub(crate) fn add_clip(parent_id: u32, low: f32, high: f32) -> u32;

        pub(super) fn reset(node_id: u32);
        pub(super) fn reset_all(node_id: u32);
        pub(super) fn clear(node_id: u32);
    }
}
