use crate::bindings as b;

#[derive(Default)]
pub struct Pad {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct Buttons {
    pub a:    bool,
    pub b:    bool,
    pub x:    bool,
    pub y:    bool,
    pub menu: bool,
}

impl Buttons {
    /// Check if any button is pressed.
    pub fn any(&self) -> bool {
        self.a || self.b || self.x || self.y || self.menu
    }

    /// Given the old state, get buttons that were not pressed but are pressed now.
    pub fn just_pressed(&self, old: &Buttons) -> Buttons {
        Buttons {
            a:    self.a && !old.a,
            b:    self.b && !old.b,
            x:    self.x && !old.x,
            y:    self.y && !old.y,
            menu: self.menu && !old.menu,
        }
    }

    /// Given the old state, get buttons that were pressed but aren't pressed now.
    pub fn just_released(&self, old: &Buttons) -> Buttons {
        Buttons {
            a:    !self.a && old.a,
            b:    !self.b && old.b,
            x:    !self.x && old.x,
            y:    !self.y && old.y,
            menu: !self.menu && old.menu,
        }
    }

    /// Given the old state, get buttons that were pressed but are still pressed now.
    pub fn held(&self, old: &Buttons) -> Buttons {
        Buttons {
            a:    self.a && old.a,
            b:    self.b && old.b,
            x:    self.x && old.x,
            y:    self.y && old.y,
            menu: self.menu && old.menu,
        }
    }
}

#[must_use]
pub fn read_pad() -> Option<Pad> {
    let raw = unsafe { b::read_pad() };
    if raw == 0xffff {
        None
    } else {
        Some(Pad {
            x: (raw >> 16) as i16 as i32,
            y: raw as i16 as i32,
        })
    }
}

#[must_use]
pub fn read_buttons() -> Buttons {
    let raw = unsafe { b::read_buttons() };
    Buttons {
        a:    has_bit_set(raw, 0),
        b:    has_bit_set(raw, 1),
        x:    has_bit_set(raw, 2),
        y:    has_bit_set(raw, 3),
        menu: has_bit_set(raw, 4),
    }
}

#[inline]
fn has_bit_set(val: u32, bit: usize) -> bool {
    (val >> bit) & 0b1 != 0
}
