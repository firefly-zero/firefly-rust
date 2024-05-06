/// The RGB value of a color in the palette.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Style of the shape.
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
            fill_color:   Color::None,
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
    /// No color (100% transparency).
    None,
    /// The first color in the palette. Typically, the darkest color.
    Dark,
    /// The second color in the palette.
    Accent,
    /// The third color in the palette.
    Secondary,
    /// The last color in the palette. Typically, the brightest, almost white, color.
    Light,
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
            1 => Ok(Color::Dark),
            2 => Ok(Color::Accent),
            3 => Ok(Color::Secondary),
            4 => Ok(Color::Light),
            _ => Err(()),
        }
    }
}

impl From<Color> for i32 {
    fn from(value: Color) -> Self {
        match value {
            Color::None => 0,
            Color::Dark => 1,
            Color::Accent => 2,
            Color::Secondary => 3,
            Color::Light => 4,
        }
    }
}

/// A mapping of colors in the image to the color palette.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ImageColors {
    pub a: Color,
    pub b: Color,
    pub c: Color,
    pub d: Color,
}

impl Default for ImageColors {
    fn default() -> Self {
        Self {
            a: Color::Dark,
            b: Color::Accent,
            c: Color::Secondary,
            d: Color::Light,
        }
    }
}

impl From<[Color; 4]> for ImageColors {
    fn from(value: [Color; 4]) -> Self {
        Self {
            a: value[0],
            b: value[1],
            c: value[2],
            d: value[3],
        }
    }
}

impl From<&[Color; 4]> for ImageColors {
    fn from(value: &[Color; 4]) -> Self {
        Self {
            a: value[0],
            b: value[1],
            c: value[2],
            d: value[3],
        }
    }
}
