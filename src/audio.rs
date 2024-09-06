#[derive(Copy, Clone)]
pub enum Pitch {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

#[derive(Copy, Clone)]
pub struct Note {
    // TODO: add constants for all notes
    // ...
    /// The pitch of the note (A, B, C#, etc).
    pub pitch: Pitch,
    /// The octave of the note in standard notation.
    pub octave: u8,
}

#[derive(Copy, Clone)]
pub struct Freq {
    f: f32,
}

impl From<Note> for Freq {
    #[expect(clippy::cast_precision_loss)]
    fn from(value: Note) -> Self {
        // https://github.com/crbulakites/hum/blob/master/src/hum_process/hum_math.rs
        // https://techlib.com/reference/musical_note_frequencies.htm
        // https://www.liutaiomottola.com/formulae/freqtab.htm
        let mut f: f32 = match value.pitch {
            Pitch::C => 16.351,
            Pitch::Cs => 17.324,
            Pitch::D => 18.354,
            Pitch::Ds => 19.445,
            Pitch::E => 20.601,
            Pitch::F => 21.827,
            Pitch::Fs => 23.124,
            Pitch::G => 24.499,
            Pitch::Gs => 25.956,
            Pitch::A => 27.5,
            Pitch::As => 29.135,
            Pitch::B => 30.868,
        };
        f *= (1 << value.octave) as f32;
        Freq { f }
    }
}

impl From<f32> for Freq {
    fn from(f: f32) -> Self {
        Self { f }
    }
}

pub struct AudioNode {
    id: u32,
}

#[expect(clippy::must_use_candidate, clippy::return_self_not_must_use)]
impl AudioNode {
    pub const ROOT: Self = Self { id: 0 };

    pub fn add_sine(&self, f: Freq, phase: f32) -> Self {
        let id = unsafe { bindings::add_sine(self.id, f.f, phase) };
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
