use crate::*;

/// System settings. Can be requested using [`get_settings`].
#[derive(Clone, Debug)]
pub struct Settings {
    /// The preferred color scheme of the player.
    pub theme: Theme,

    /// The configured interface language.
    pub language: Language,

    /// If true, the screen is rotated 180 degrees.
    ///
    /// In other words, the player holds the device upside-down.
    /// The touchpad is now on the right and the buttons are on the left.
    pub rotate_screen: bool,

    /// The player has photosensitivity. The app should avoid any rapid flashes.
    pub reduce_flashing: bool,

    /// The player wants increased contrast for colors.
    ///
    /// If set, the black and white colors in the default
    /// palette are adjusted automatically. All other colors
    /// in the default palette or all colors in a custom palette
    /// should be adjusted by the app.
    pub contrast: bool,

    /// If true, the player wants to see easter eggs, holiday effects, and weird jokes.
    pub easter_eggs: bool,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
pub enum Language {
    /// en 🇬🇧 💂
    #[default]
    English,
    /// nl 🇳🇱 🧀
    Dutch,
    /// fr 🇫🇷 🥐
    French,
    /// de 🇩🇪 🥨
    German,
    /// it 🇮🇹 🍕
    Italian,
    /// pl 🇵🇱 🥟
    Polish,
    /// ru 🇷🇺 🪆
    Russian,
    // sp 🇪🇸 🐂
    Spanish,
    // tk 🇹🇷 🕌
    Turkish,
    /// ua 🇺🇦 ✊
    Ukrainian,
    /// tp 🇨🇦 🙂
    TokiPona,
}

impl Language {
    #[must_use]
    pub fn from_code(b: [u8; 2]) -> Option<Self> {
        let code = match b {
            [b'e', b'n'] => Self::English,
            [b'n', b'l'] => Self::Dutch,
            [b'f', b'r'] => Self::French,
            [b'd', b'e'] => Self::German,
            [b'i', b't'] => Self::Italian,
            [b'p', b'o'] => Self::Polish,
            [b'r', b'u'] => Self::Russian,
            [b's', b'p'] => Self::Spanish,
            [b't', b'p'] => Self::TokiPona,
            [b't', b'k'] => Self::Turkish,
            [b'u', b'a'] => Self::Ukrainian,
            _ => return None,
        };
        Some(code)
    }

    #[must_use]
    pub fn code_array(&self) -> [u8; 2] {
        match self {
            Self::English => [b'e', b'n'],
            Self::Dutch => [b'n', b'l'],
            Self::French => [b'f', b'r'],
            Self::German => [b'd', b'e'],
            Self::Italian => [b'i', b't'],
            Self::Polish => [b'p', b'o'],
            Self::Russian => [b'r', b'u'],
            Self::Spanish => [b's', b'p'],
            Self::TokiPona => [b't', b'p'],
            Self::Turkish => [b't', b'k'],
            Self::Ukrainian => [b'u', b'a'],
        }
    }

    #[must_use]
    pub fn code_str(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Dutch => "nl",
            Self::French => "fr",
            Self::German => "de",
            Self::Italian => "it",
            Self::Polish => "po",
            Self::Russian => "ru",
            Self::Spanish => "sp",
            Self::TokiPona => "tp",
            Self::Turkish => "tk",
            Self::Ukrainian => "ua",
        }
    }

    /// ISO 8859 encoding slug for the language.
    ///
    /// Useful for dynamically loading the correct font for the given language.
    #[must_use]
    pub fn encoding(&self) -> &'static str {
        match self {
            // Just like English, Dutch has very little non-ASCII characters
            // which can be avoided in translations to make it possible to stick
            // to the smaller fonts.
            Self::English | Self::Dutch | Self::TokiPona => "ascii",
            Self::Italian | Self::Spanish => "iso_8859_1",
            Self::German | Self::French => "iso_8859_2",
            Self::Polish => "iso_8859_13",
            Self::Russian | Self::Ukrainian => "iso_8859_5",
            Self::Turkish => "iso_8859_9",
        }
    }
}

/// The preferred color scheme of the peer.
///
/// Can be useful for:
///
/// * Making UI that matches the system UI.
/// * Preventing image flashes by making the UI background
///   the same as in the system UI.
/// * Providing and auto-switching the dark and light mode.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub id: u8,
    /// The main color of text and boxes.
    pub primary: Color,
    // The color of disable options, muted text, etc.
    pub secondary: Color,
    // The color of important elements, active options, etc.
    pub accent: Color,
    // The background color, the most contrast color to primary.
    pub bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            id: 0,
            primary: Color::Black,
            secondary: Color::LightGray,
            accent: Color::Green,
            bg: Color::White,
        }
    }
}

/// Log a debug message.
pub fn log_debug(t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        bindings::log_debug(ptr, len);
    }
}

/// Log an error message.
pub fn log_error(t: &str) {
    let ptr = t.as_ptr() as u32;
    let len = t.len() as u32;
    unsafe {
        bindings::log_error(ptr, len);
    }
}

/// Set the random seed.
pub fn set_seed(seed: u32) {
    unsafe {
        bindings::set_seed(seed);
    }
}

/// Get a random value.
#[must_use]
pub fn get_random() -> u32 {
    unsafe { bindings::get_random() }
}

/// Get the Peer's name.
///
/// The name is guaranteed to be a valid ASCII string
/// and have between 1 and 16 characters.
#[cfg(feature = "alloc")]
#[must_use]
pub fn get_name_buf(p: Peer) -> alloc::string::String {
    let mut buf = [0u8; 16];
    let name = get_name(p, &mut buf);
    alloc::string::String::from(name)
}

/// Get the Peer's name.
///
/// The name is guaranteed to be a valid ASCII string
/// and have between 1 and 16 characters.
#[must_use]
pub fn get_name(p: Peer, buf: &mut [u8; 16]) -> &str {
    let ptr = buf.as_ptr() as u32;
    let len = unsafe { bindings::get_name(u32::from(p.0), ptr) };
    let buf = &buf[..len as usize];
    unsafe { core::str::from_utf8_unchecked(buf) }
}

/// Get the peer's system settings.
#[must_use]
pub fn get_settings(p: Peer) -> Settings {
    let raw = unsafe { bindings::get_settings(u32::from(p.0)) };
    let code = [(raw >> 8) as u8, raw as u8];
    let language = Language::from_code(code).unwrap_or_default();
    let flags = raw >> 16;
    let theme = raw >> 32;
    let theme = Theme {
        id: theme as u8,
        primary: Color::from((raw >> 20) as u8),
        secondary: Color::from((raw >> 16) as u8),
        accent: Color::from((raw >> 12) as u8),
        bg: Color::from((raw >> 8) as u8),
    };
    Settings {
        theme,
        language,
        rotate_screen: (flags & 0b0001) != 0,
        reduce_flashing: (flags & 0b0010) != 0,
        contrast: (flags & 0b0100) != 0,
        easter_eggs: (flags & 0b1000) != 0,
    }
}

/// Exit the app after the current update is finished.
pub fn quit() {
    unsafe { bindings::quit() }
}

mod bindings {
    #[link(wasm_import_module = "misc")]
    unsafe extern "C" {
        pub(crate) unsafe fn log_debug(ptr: u32, len: u32);
        pub(crate) unsafe fn log_error(ptr: u32, len: u32);
        pub(crate) unsafe fn set_seed(seed: u32);
        pub(crate) unsafe fn get_random() -> u32;
        pub(crate) unsafe fn get_name(idx: u32, ptr: u32) -> u32;
        pub(crate) unsafe fn get_settings(idx: u32) -> u64;
        pub(crate) unsafe fn quit();
    }
}
