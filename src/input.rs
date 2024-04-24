use crate::bindings as b;
use crate::net::Player;

const DPAD_THRESHOLD: i32 = 100;

/// A finger position on the touch pad.
///
/// Both x and y are somewhere the range between -1000 and 1000 (both ends included).
/// The 1000 x is on the right, the 1000 y is on the top.
#[derive(Default)]
pub struct Pad {
    pub x: i32,
    pub y: i32,
}

impl Pad {
    /// The right-most value of x.
    pub const MAX_X: i32 = 1000;
    /// The top-most value of y.
    pub const MAX_Y: i32 = 1000;
    /// The left-most value of x.
    pub const MIN_X: i32 = -1000;
    /// The bottom-most value of y.
    pub const MIN_Y: i32 = -1000;

    /// Represent the pad values as a directional pad.
    pub fn as_dpad(&self) -> DPad {
        DPad {
            left:  self.x <= -DPAD_THRESHOLD,
            right: self.x >= DPAD_THRESHOLD,
            down:  self.y <= -DPAD_THRESHOLD,
            up:    self.y >= DPAD_THRESHOLD,
        }
    }
}

/// DPad-like representation of the [Pad].
///
/// Constructed with [Pad::as_dpad]. Useful for simple games and ports.
/// The middle of the pad is a "dead zone" pressing which will not activate any direction.
///
/// Invariant: it's not possible for opposite directions (left and right, or down and up)
/// to be active at the same time. However, it's possible for heighboring directions
/// (like up and right) to be active at the same time if the player presses a diagonal.
#[derive(Default)]
pub struct DPad {
    pub left:  bool,
    pub right: bool,
    pub up:    bool,
    pub down:  bool,
}

impl From<Pad> for DPad {
    fn from(value: Pad) -> Self {
        value.as_dpad()
    }
}

impl DPad {
    /// Given the old state, get directions that were not pressed but are pressed now.
    pub fn just_pressed(&self, old: &Self) -> Self {
        Self {
            left:  self.left && !old.left,
            right: self.right && !old.right,
            up:    self.up && !old.up,
            down:  self.down && !old.down,
        }
    }

    /// Given the old state, get directions that were pressed but aren't pressed now.
    pub fn just_released(&self, old: &Self) -> Self {
        Self {
            left:  !self.left && old.left,
            right: !self.right && old.right,
            up:    !self.up && old.up,
            down:  !self.down && old.down,
        }
    }

    /// Given the old state, get directions that were pressed but are still pressed now.
    pub fn held(&self, old: &Self) -> Self {
        Self {
            left:  self.left && old.left,
            right: self.right && old.right,
            up:    self.up && old.up,
            down:  self.down && old.down,
        }
    }
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
    pub fn just_pressed(&self, old: &Self) -> Self {
        Self {
            a:    self.a && !old.a,
            b:    self.b && !old.b,
            x:    self.x && !old.x,
            y:    self.y && !old.y,
            menu: self.menu && !old.menu,
        }
    }

    /// Given the old state, get buttons that were pressed but aren't pressed now.
    pub fn just_released(&self, old: &Self) -> Self {
        Self {
            a:    !self.a && old.a,
            b:    !self.b && old.b,
            x:    !self.x && old.x,
            y:    !self.y && old.y,
            menu: !self.menu && old.menu,
        }
    }

    /// Given the old state, get buttons that were pressed but are still pressed now.
    pub fn held(&self, old: &Self) -> Self {
        Self {
            a:    self.a && old.a,
            b:    self.b && old.b,
            x:    self.x && old.x,
            y:    self.y && old.y,
            menu: self.menu && old.menu,
        }
    }
}

#[must_use]
pub fn read_pad(player: Player) -> Option<Pad> {
    let raw = unsafe { b::read_pad(player.into()) };
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
pub fn read_buttons(player: Player) -> Buttons {
    let raw = unsafe { b::read_buttons(player.into()) };
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
