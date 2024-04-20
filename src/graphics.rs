use crate::bindings as b;

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
