use core::marker::PhantomData;

use super::*;

pub struct Root {}
pub struct Sine {}
pub struct Gain {}
pub struct Square {}
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

    pub fn add_gain(&self, lvl: f32) -> Node<Gain> {
        let id = unsafe { bindings::add_gain(self.id, lvl) };
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
    pub fn modulate_level<M: Modulator>(&self, m: M) {
        m.modulate(self.id, 0);
    }
}

mod bindings {
    #[link(wasm_import_module = "audio")]
    extern {
        pub(super) fn add_sine(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) fn add_gain(parent_id: u32, lvl: f32) -> u32;
        pub(super) fn add_square(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(super) fn add_noise(parent_id: u32, seed: i32) -> u32;
        pub(super) fn add_empty(parent_id: u32) -> u32;
        pub(super) fn add_zero(parent_id: u32) -> u32;
        pub(super) fn reset(node_id: u32);
        pub(super) fn reset_all(node_id: u32);
        pub(super) fn clear(node_id: u32);
    }
}
