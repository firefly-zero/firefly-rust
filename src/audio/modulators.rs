use super::*;

pub trait Modulator {
    fn modulate(self, node_id: u32, param: u32);
}

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

mod bindings {
    #[link(wasm_import_module = "audio")]
    extern {
        pub(crate) fn mod_linear(
            node_id: u32,
            param: u32,
            start: f32,
            end: f32,
            start_at: u32,
            end_at: u32,
        );
    }
}
