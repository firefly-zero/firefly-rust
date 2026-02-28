//! Read inputs: touch pad, buttons, accelerometer.

use crate::*;

const DPAD4_THRESHOLD: i32 = 300;
const DPAD8_THRESHOLD: i32 = 300;

/// A finger position on the touch pad.
///
/// Both x and y are somewhere the range between -1000 and 1000 (both ends included).
/// The 1000 x is on the right, the 1000 y is on the top.
///
/// Note that the y coordinate is flipped compared to the screen coordinates.
/// While the display coordinates follow the text reading direction,
/// the touchpad coordinates follow the directions used in geometry.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Pad {
    pub x: i32,
    pub y: i32,
}

impl Pad {
    /// The top-right corner of the pad's bounding box.
    pub const MAX: Pad = Pad { x: 1000, y: 1000 };
    /// The bottom-left corner of the pad's bounding box.
    pub const MIN: Pad = Pad { x: -1000, y: -1000 };

    /// Represent the pad values as an 4-directional pad.
    ///
    /// Use [`Pad::as_dpad8`] instead if you want to also allow diagonal movement.
    #[must_use]
    pub fn as_dpad4(&self) -> DPad4 {
        let x = self.x;
        let y = self.y;
        if y > DPAD4_THRESHOLD && y > x.abs() {
            DPad4::Up
        } else if y < -DPAD4_THRESHOLD && -y > x.abs() {
            DPad4::Down
        } else if x > DPAD4_THRESHOLD && x > y.abs() {
            DPad4::Right
        } else if x < -DPAD4_THRESHOLD && -x > y.abs() {
            DPad4::Left
        } else {
            DPad4::None
        }
    }

    /// Represent the pad values as an 8-directional pad.
    ///
    /// Use [`Pad::as_dpad4`] instead if you don't want to allow diagonal movement.
    #[must_use]
    pub fn as_dpad8(&self) -> DPad8 {
        DPad8 {
            left: self.x <= -DPAD8_THRESHOLD,
            right: self.x >= DPAD8_THRESHOLD,
            down: self.y <= -DPAD8_THRESHOLD,
            up: self.y >= DPAD8_THRESHOLD,
        }
    }

    /// The distance from the pad center to the touch point.
    #[must_use]
    pub fn radius(self) -> f32 {
        let r = self.x * self.x + self.y * self.y;
        #[expect(clippy::cast_precision_loss)]
        math::sqrt(r as f32)
    }

    /// The angle of the [polar coordinate] of the touch point.
    ///
    /// [polar coordinate]: https://en.wikipedia.org/wiki/Polar_coordinate_system
    #[must_use]
    pub fn azimuth(self) -> Angle {
        if self.x == 0 && self.y == 0 {
            return Angle::ZERO;
        }
        #[expect(clippy::cast_precision_loss)]
        let r = math::atan2(self.y as f32, self.x as f32);
        Angle::from_radians(r)
    }
}

impl From<Pad> for Point {
    fn from(value: Pad) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Point> for Pad {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Pad> for Size {
    fn from(value: Pad) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}

impl From<Size> for Pad {
    fn from(value: Size) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

/// 8-directional DPad-like representation of the [`Pad`].
///
/// Constructed with [`Pad::as_dpad8`]. Useful for simple games and ports.
/// The middle of the pad is a "dead zone" pressing which will not activate any direction.
///
/// Implements all the same methods as [`DPad4`].
///
/// Invariant: it's not possible for opposite directions (left and right, or down and up)
/// to be active at the same time. However, it's possible for heighboring directions
/// (like up and right) to be active at the same time if the player presses a diagonal.
///
/// For completeness, here is the full list of possible states:
///
/// * right
/// * right and up
/// * up
/// * left and up
/// * left
/// * left and down
/// * down
/// * right and down
/// * none
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DPad8 {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl DPad8 {
    /// Check if any direction is pressed.
    #[must_use]
    pub fn any(&self) -> bool {
        self.left || self.right || self.up || self.down
    }

    /// Given the old state, get directions that were not pressed but are pressed now.
    #[must_use]
    pub fn just_pressed(&self, old: &Self) -> Self {
        Self {
            left: self.left && !old.left,
            right: self.right && !old.right,
            up: self.up && !old.up,
            down: self.down && !old.down,
        }
    }

    /// Given the old state, get directions that were pressed but aren't pressed now.
    #[must_use]
    pub fn just_released(&self, old: &Self) -> Self {
        Self {
            left: !self.left && old.left,
            right: !self.right && old.right,
            up: !self.up && old.up,
            down: !self.down && old.down,
        }
    }

    /// Given the old state, get directions that were pressed and are still pressed now.
    #[must_use]
    pub fn held(&self, old: &Self) -> Self {
        Self {
            left: self.left && old.left,
            right: self.right && old.right,
            up: self.up && old.up,
            down: self.down && old.down,
        }
    }
}

impl From<Pad> for DPad8 {
    fn from(value: Pad) -> Self {
        value.as_dpad8()
    }
}

impl From<Option<DPad8>> for DPad8 {
    fn from(value: Option<DPad8>) -> Self {
        value.unwrap_or_default()
    }
}

impl From<DPad4> for DPad8 {
    fn from(value: DPad4) -> Self {
        let mut pad = Self::default();
        match value {
            DPad4::None => {}
            DPad4::Left => pad.left = true,
            DPad4::Right => pad.right = true,
            DPad4::Up => pad.up = true,
            DPad4::Down => pad.down = true,
        }
        pad
    }
}

/// 4-directional DPad-like representation of the [`Pad`].
///
/// Constructed with [`Pad::as_dpad4`]. Useful for simple games and ports.
/// The middle of the pad is a "dead zone" pressing which will not activate any direction.
///
/// Implements all the same methods as [`DPad8`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum DPad4 {
    #[default]
    None,
    Left,
    Right,
    Up,
    Down,
}

impl DPad4 {
    /// Check if any direction is pressed.
    #[must_use]
    pub fn any(self) -> bool {
        self != Self::None
    }

    /// Given the old state, get directions that were not pressed but are pressed now.
    #[must_use]
    pub fn just_pressed(self, old: Self) -> Self {
        if self == old { Self::None } else { self }
    }

    /// Given the old state, get directions that were pressed but aren't pressed now.
    #[must_use]
    pub fn just_released(self, old: Self) -> Self {
        if self == old { Self::None } else { old }
    }

    /// Given the old state, get directions that were pressed and are still pressed now.
    #[must_use]
    pub fn held(self, old: Self) -> Self {
        if self == old { self } else { Self::None }
    }
}

impl From<Pad> for DPad4 {
    fn from(value: Pad) -> Self {
        value.as_dpad4()
    }
}

impl From<Option<DPad4>> for DPad4 {
    fn from(value: Option<DPad4>) -> Self {
        value.unwrap_or_default()
    }
}

/// State of the buttons.
#[derive(Default, Clone, Copy)]
pub struct Buttons {
    /// South. The bottom button, like A on the X-Box controller.
    ///
    /// Typically used for confirmation, main action, jump, etc.
    pub s: bool,

    /// East. The right button, like B on the X-Box controller.
    ///
    /// Typically used for cancellation, going to previous screen, etc.
    pub e: bool,

    /// West. The left button, like X on the X-Box controller.
    ///
    /// Typically used for attack.
    pub w: bool,

    /// North. The top button, like Y on the X-Box controller.
    ///
    /// Typically used for a secondary action, like charged attack.
    pub n: bool,

    /// The menu button, almost always handled by the runtime.
    pub menu: bool,
}

impl Buttons {
    /// Check if any button is pressed.
    #[must_use]
    pub fn any(&self) -> bool {
        self.s || self.e || self.w || self.n || self.menu
    }

    /// Given the old state, get buttons that were not pressed but are pressed now.
    #[must_use]
    pub fn just_pressed(&self, old: &Self) -> Self {
        Self {
            s: self.s && !old.s,
            e: self.e && !old.e,
            w: self.w && !old.w,
            n: self.n && !old.n,
            menu: self.menu && !old.menu,
        }
    }

    /// Given the old state, get buttons that were pressed but aren't pressed now.
    #[must_use]
    pub fn just_released(&self, old: &Self) -> Self {
        Self {
            s: !self.s && old.s,
            e: !self.e && old.e,
            w: !self.w && old.w,
            n: !self.n && old.n,
            menu: !self.menu && old.menu,
        }
    }

    /// Given the old state, get buttons that were pressed but are still pressed now.
    #[must_use]
    pub fn held(&self, old: &Self) -> Self {
        Self {
            s: self.s && old.s,
            e: self.e && old.e,
            w: self.w && old.w,
            n: self.n && old.n,
            menu: self.menu && old.menu,
        }
    }
}

/// Get the current touch pad state.
#[must_use]
pub fn read_pad(p: Peer) -> Option<Pad> {
    let p = u32::from(p.0);
    let raw = unsafe { bindings::read_pad(p) };
    if raw == 0xffff {
        None
    } else {
        Some(Pad {
            x: i32::from((raw >> 16) as i16),
            y: i32::from(raw as i16),
        })
    }
}

/// Get the currently pressed buttons.
#[must_use]
pub fn read_buttons(p: Peer) -> Buttons {
    let p = u32::from(p.0);
    let raw = unsafe { bindings::read_buttons(p) };
    Buttons {
        s: has_bit_set(raw, 0),
        e: has_bit_set(raw, 1),
        w: has_bit_set(raw, 2),
        n: has_bit_set(raw, 3),
        menu: has_bit_set(raw, 4),
    }
}

/// Check if the given i32 value has the given bit set.
#[inline]
fn has_bit_set(val: u32, bit: usize) -> bool {
    (val >> bit) & 0b1 != 0
}

mod bindings {
    #[link(wasm_import_module = "input")]
    unsafe extern "C" {
        pub(crate) fn read_pad(peer: u32) -> u32;
        pub(crate) fn read_buttons(peer: u32) -> u32;
    }
}
