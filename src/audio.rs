pub struct AudioNode {
    id: u32,
}

#[allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]
impl AudioNode {
    pub const ROOT: Self = Self { id: 0 };

    pub fn add_sine(&self, freq: f32, phase: f32) -> Self {
        let id = unsafe { bindings::add_sine(self.id, freq, phase) };
        Self { id }
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

mod bindings {
    #[link(wasm_import_module = "audio")]
    extern {
        pub(crate) fn add_sine(parent_id: u32, freq: f32, phase: f32) -> u32;
        pub(crate) fn reset(node_id: u32);
        pub(crate) fn reset_all(node_id: u32);
        pub(crate) fn clear(node_id: u32);
    }
}
