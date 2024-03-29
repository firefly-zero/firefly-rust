use crate::graphics_bindings as b;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width:  u32,
    pub height: u32,
}

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Style {
    pub fill_color:   u8,
    pub stroke_color: u8,
    pub stroke_width: u32,
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
            s.stroke_width as i32,
        )
    }
}
