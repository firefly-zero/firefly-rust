/// The RGB value of a color in the palette.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    /// Convert the style to a line style.
    ///
    /// [`LineStyle`] is the same as [Style] except it doesn't have a fill color.
    #[must_use]
    pub fn as_line_style(&self) -> LineStyle {
        LineStyle {
            color: self.stroke_color,
            width: self.stroke_width,
        }
    }
}

/// The same as [Style] but without a fill color (only stroke color and width).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LineStyle {
    pub color: Color,
    pub width: i32,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Color {
    None,
    Black,
    Purple,
    Red,
    Orange,
    Yellow,
    LightGreen,
    Green,
    DarkGreen,
    DarkBlue,
    Blue,
    LightBlue,
    Cyan,
    White,
    LightGray,
    Gray,
    DarkGray,
}

impl Default for Color {
    fn default() -> Self {
        Self::None
    }
}

impl TryFrom<usize> for Color {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::None),
            1 => Ok(Color::Black),
            2 => Ok(Color::Purple),
            3 => Ok(Color::Red),
            4 => Ok(Color::Orange),
            5 => Ok(Color::Yellow),
            6 => Ok(Color::LightGreen),
            7 => Ok(Color::Green),
            8 => Ok(Color::DarkGreen),
            9 => Ok(Color::DarkBlue),
            10 => Ok(Color::Blue),
            11 => Ok(Color::LightBlue),
            12 => Ok(Color::Cyan),
            13 => Ok(Color::White),
            14 => Ok(Color::LightGray),
            15 => Ok(Color::Gray),
            16 => Ok(Color::DarkGray),
            _ => Err(()),
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
