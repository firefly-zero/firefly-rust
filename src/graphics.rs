use crate::bindings as b;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width:  i32,
    pub height: i32,
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Style {
    pub fill_color:   u8,
    pub stroke_color: Color,
    pub stroke_width: i32,
}

pub struct Color(u8);

impl Color {
    pub const ACCENT: Color = Color(2);
    pub const DARK: Color = Color(1);
    pub const LIGHT: Color = Color(4);
    pub const NONE: Color = Color(0);
    pub const SECONDARY: Color = Color(3);
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        debug_assert!(value <= 4);
        Self(value)
    }
}

impl From<Color> for i32 {
    fn from(value: Color) -> Self {
        value.0 as i32
    }
}

pub fn draw_triangle(a: Point, b: Point, c: Point, s: Style) {
    unsafe {
        b::draw_triangle(
            a.x,
            a.y,
            b.x,
            b.y,
            c.x,
            c.y,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        )
    }
}
