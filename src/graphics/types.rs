/// The RGB value of a color in the palette.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RGB {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
}

impl RGB {
    /// Create a new RGB color.
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Style of a shape.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Style {
    /// The color to use to fill the shape.
    pub fill_color: Color,

    /// The color to use for the shape stroke.
    pub stroke_color: Color,

    /// The width of the shape stroke.
    ///
    /// If zero, a solid shape without a stroke will be drawn.
    pub stroke_width: i32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill_color: Color::None,
            stroke_color: Color::None,
            stroke_width: 1,
        }
    }
}

impl Style {
    /// Create a shape style filled with a color and without a stroke.
    #[must_use]
    pub const fn solid(c: Color) -> Self {
        Self {
            fill_color: c,
            stroke_color: Color::None,
            stroke_width: 0,
        }
    }

    /// Create a shape style with a stroke and no fill color (transparent body).
    #[must_use]
    pub const fn outlined(c: Color, w: i32) -> Self {
        Self {
            fill_color: Color::None,
            stroke_color: c,
            stroke_width: w,
        }
    }

    /// Convert the style to a line style.
    ///
    /// [`LineStyle`] is the same as [Style] except it doesn't have a fill color.
    #[must_use]
    pub const fn as_line_style(&self) -> LineStyle {
        LineStyle {
            color: self.stroke_color,
            width: self.stroke_width,
        }
    }
}

impl From<LineStyle> for Style {
    fn from(value: LineStyle) -> Self {
        Self {
            fill_color: Color::None,
            stroke_color: value.color,
            stroke_width: value.width,
        }
    }
}

/// The same as [Style] but without a fill color (only stroke color and width).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LineStyle {
    /// The line stroke color.
    pub color: Color,
    /// The line stroke width.
    pub width: i32,
}

impl LineStyle {
    /// Create a new style for a line.
    #[must_use]
    pub const fn new(c: Color, w: i32) -> Self {
        Self { color: c, width: w }
    }
}

impl From<Style> for LineStyle {
    fn from(value: Style) -> Self {
        Self {
            color: value.stroke_color,
            width: value.stroke_width,
        }
    }
}

/// A pointer to a color in the color palette.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Color {
    /// No color (100% transparency).
    #[default]
    None,
    /// Black color: #1A1C2C.
    Black,
    /// Purple color: #5D275D.
    Purple,
    /// Red color: #B13E53.
    Red,
    /// Orange color: #EF7D57.
    Orange,
    /// Yellow color: #FFCD75.
    Yellow,
    /// Light green color: #A7F070.
    LightGreen,
    /// Green color: #38B764.
    Green,
    /// Dark green color: #257179.
    DarkGreen,
    /// Dark blue color: #29366F.
    DarkBlue,
    /// Blue color: #3B5DC9.
    Blue,
    /// Light blue color: #41A6F6.
    LightBlue,
    /// Cyan color: #73EFF7.
    Cyan,
    /// White color: #F4F4F4.
    White,
    /// Light gray color: #94B0C2.
    LightGray,
    /// Gray color: #566C86.
    Gray,
    /// Dark gray color: #333C57.
    DarkGray,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        Self::from(i32::from(value))
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        match value {
            1 => Color::Black,
            2 => Color::Purple,
            3 => Color::Red,
            4 => Color::Orange,
            5 => Color::Yellow,
            6 => Color::LightGreen,
            7 => Color::Green,
            8 => Color::DarkGreen,
            9 => Color::DarkBlue,
            10 => Color::Blue,
            11 => Color::LightBlue,
            12 => Color::Cyan,
            13 => Color::White,
            14 => Color::LightGray,
            15 => Color::Gray,
            16 => Color::DarkGray,
            _ => Color::None,
        }
    }
}

impl From<Color> for i32 {
    fn from(value: Color) -> Self {
        match value {
            Color::None => 0,
            Color::Black => 1,
            Color::Purple => 2,
            Color::Red => 3,
            Color::Orange => 4,
            Color::Yellow => 5,
            Color::LightGreen => 6,
            Color::Green => 7,
            Color::DarkGreen => 8,
            Color::DarkBlue => 9,
            Color::Blue => 10,
            Color::LightBlue => 11,
            Color::Cyan => 12,
            Color::White => 13,
            Color::LightGray => 14,
            Color::Gray => 15,
            Color::DarkGray => 16,
        }
    }
}

impl From<Option<Color>> for Color {
    fn from(value: Option<Color>) -> Self {
        value.unwrap_or(Color::None)
    }
}
