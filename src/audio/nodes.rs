use core::marker::PhantomData;

use super::*;

pub struct Root {}
pub struct Sine {}

pub struct Node<F> {
    id: u32,
    _flavor: PhantomData<F>,
}

pub const OUT: Node<Root> = Node {
    id: 0,
    _flavor: PhantomData,
};

#[expect(clippy::must_use_candidate)]
impl<F> Node<F> {
    pub fn add_sine(&self, f: Freq, phase: f32) -> Node<Sine> {
        let id = unsafe { bindings::add_sine(self.id, f.0, phase) };
        Node {
            id,
            _flavor: PhantomData,
        }
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

impl Node<Sine> {
    pub fn mod_freq(&self) {
        todo!()
    }
}

mod bindings {
    #[link(wasm_import_module = "audio")]
    extern {
        pub(crate) fn add_sine(parent_id: u32, freq: f32, phase: f32) -> u32;
        // pub(crate) fn add_square(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(crate) fn add_gain(lvl: f32) -> u32;

        fn mod_linear(node_id: u32, param: u32, start: f32, end: f32, start_at: u32, end_at: u32);

        pub(crate) fn reset(node_id: u32);
        pub(crate) fn reset_all(node_id: u32);
        pub(crate) fn clear(node_id: u32);
    }
}
