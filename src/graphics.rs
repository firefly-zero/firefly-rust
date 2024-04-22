use crate::bindings as b;
use crate::fs::{Font, Image, SubImage};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Size of a bounding box for a shape.
pub struct Size {
    pub width:  i32,
    pub height: i32,
}

/// The RGB value of a color in the palette.
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Style {
    pub fill_color:   Color,
    pub stroke_color: Color,
    pub stroke_width: i32,
}

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

pub struct Color(u8);

// The formatter reorders constants alphabetically, which doesn't make sense in this case.
#[rustfmt::skip]
impl Color {
    /// The first color in the palette. Typically, the darkest color.
    pub const DARK: Color = Color(1);

    /// The second color in the palette.
    pub const ACCENT: Color = Color(2);

    /// The third color in the palette.
    pub const SECONDARY: Color = Color(3);

    /// The last color in the palette. Typically, the brightest, almost white, color.
    pub const LIGHT: Color = Color(4);

    /// No color (100% transparency).
    pub const NONE: Color = Color(0);
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

pub struct ImageColors {
    a: Color,
    b: Color,
    c: Color,
    d: Color,
}

impl Default for ImageColors {
    fn default() -> Self {
        Self {
            a: Color::DARK,
            b: Color::ACCENT,
            c: Color::SECONDARY,
            d: Color::LIGHT,
        }
    }
}

pub fn get_screen_size() -> Size {
    let raw = unsafe { b::get_screen_size() };
    Size {
        width:  (raw >> 16) & 0xffff,
        height: raw & 0xffff,
    }
}

pub fn clear_screen(c: Color) {
    unsafe {
        b::clear_screen(c.into());
    }
}

pub fn set_color(c: Color, v: RGB) {
    unsafe {
        b::set_color(c.into(), v.r.into(), v.g.into(), v.b.into());
    }
}

pub fn set_colors(dark: RGB, accent: RGB, secondary: RGB, light: RGB) {
    unsafe {
        b::set_colors(
            dark.r.into(),
            dark.g.into(),
            dark.b.into(),
            accent.r.into(),
            accent.g.into(),
            accent.b.into(),
            secondary.r.into(),
            secondary.g.into(),
            secondary.b.into(),
            light.r.into(),
            light.g.into(),
            light.b.into(),
        )
    }
}

pub fn draw_point(p: Point, c: Color) {
    unsafe {
        b::draw_point(p.x, p.y, c.into());
    }
}

pub fn draw_line(a: Point, b: Point, s: LineStyle) {
    unsafe {
        b::draw_line(a.x, a.y, b.x, b.y, s.color.into(), s.width);
    }
}

pub fn draw_rect(p: Point, b: Size, s: Style) {
    unsafe {
        b::draw_rect(
            p.x,
            p.y,
            b.width,
            b.height,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

pub fn draw_rounded_rect(p: Point, b: Size, corner: Size, s: Style) {
    unsafe {
        b::draw_rounded_rect(
            p.x,
            p.y,
            b.width,
            b.height,
            corner.width,
            corner.height,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

pub fn draw_circle(p: Point, d: i32, s: Style) {
    unsafe {
        b::draw_circle(
            p.x,
            p.y,
            d,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

pub fn draw_ellipse(p: Point, b: Size, s: Style) {
    unsafe {
        b::draw_ellipse(
            p.x,
            p.y,
            b.width,
            b.height,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
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
        );
    }
}

pub fn draw_arc(p: Point, d: i32, angle_start: i32, angle_sweep: i32, s: Style) {
    unsafe {
        b::draw_arc(
            p.x,
            p.y,
            d,
            angle_start,
            angle_sweep,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

pub fn draw_sector(p: Point, d: i32, angle_start: i32, angle_sweep: i32, s: Style) {
    unsafe {
        b::draw_sector(
            p.x,
            p.y,
            d,
            angle_start,
            angle_sweep,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

pub fn draw_text(t: &str, f: &Font, p: Point, c: Color) {
    let text_ptr = t.as_ptr();
    let text_len = t.len();
    let font_ptr = f.raw.as_ptr();
    let font_len = f.raw.len();
    unsafe {
        b::draw_text(
            text_ptr as i32,
            text_len as i32,
            font_ptr as i32,
            font_len as i32,
            p.x,
            p.y,
            c.into(),
        );
    }
}

pub fn draw_image(i: Image, p: Point, c: ImageColors) {
    let ptr = i.raw.as_ptr();
    let len = i.raw.len();
    unsafe {
        b::draw_image(
            ptr as i32,
            len as i32,
            p.x,
            p.y,
            c.a.into(),
            c.b.into(),
            c.c.into(),
            c.d.into(),
        );
    }
}

pub fn draw_sub_image(i: SubImage, p: Point, c: ImageColors) {
    let ptr = i.raw.as_ptr();
    let len = i.raw.len();
    unsafe {
        b::draw_sub_image(
            ptr as i32,
            len as i32,
            p.x,
            p.y,
            i.point.x,
            i.point.y,
            i.size.width,
            i.size.height,
            c.a.into(),
            c.b.into(),
            c.c.into(),
            c.d.into(),
        );
    }
}
