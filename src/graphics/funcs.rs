use super::{bindings as b, *};
use crate::fs::{Font, Image, SubImage};

/// Fill the whole frame with the given color.
pub fn clear_screen(c: Color) {
    unsafe {
        b::clear_screen(c.into());
    }
}

/// Set a color value in the palette.
pub fn set_color(c: Color, v: RGB) {
    unsafe {
        b::set_color(c.into(), v.r.into(), v.g.into(), v.b.into());
    }
}

/// Set the color palette.
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
        );
    }
}

/// Set a single point (1 pixel is scaling is 1) on the frame.
pub fn draw_point(p: Point, c: Color) {
    unsafe {
        b::draw_point(p.x, p.y, c.into());
    }
}

/// Draw a straight line from point a to point b.
pub fn draw_line(a: Point, b: Point, s: LineStyle) {
    unsafe {
        b::draw_line(a.x, a.y, b.x, b.y, s.color.into(), s.width);
    }
}

/// Draw a rectangle filling the given bounding box.
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

/// Draw a rectangle with rounded corners.
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

/// Draw a circle with the given diameter.
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

/// Draw an ellipse (oval).
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

/// Draw a triangle.
///
/// The order of points doesn't matter.
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

/// Draw an arc.
pub fn draw_arc(p: Point, d: i32, start: Angle, sweep: Angle, s: Style) {
    unsafe {
        b::draw_arc(
            p.x,
            p.y,
            d,
            start.0,
            sweep.0,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

/// Draw a sector.
pub fn draw_sector(p: Point, d: i32, start: Angle, sweep: Angle, s: Style) {
    unsafe {
        b::draw_sector(
            p.x,
            p.y,
            d,
            start.0,
            sweep.0,
            s.fill_color.into(),
            s.stroke_color.into(),
            s.stroke_width,
        );
    }
}

/// Render text using the given font.
///
/// Unlike in the other drawing functions, here [Point] points not to the top-left corner
/// but to the baseline start position.
pub fn draw_text(t: &str, f: &Font, p: Point, c: Color) {
    let text_ptr = t.as_ptr();
    let text_len = t.len();
    let font_ptr = f.raw.as_ptr();
    let font_len = f.raw.len();
    unsafe {
        b::draw_text(
            text_ptr as u32,
            text_len as u32,
            font_ptr as u32,
            font_len as u32,
            p.x,
            p.y,
            c.into(),
        );
    }
}

/// Render an image using the given colors.
pub fn draw_image(i: &Image, p: Point) {
    let ptr = i.raw.as_ptr();
    let len = i.raw.len();
    unsafe {
        b::draw_image(ptr as u32, len as u32, p.x, p.y);
    }
}

/// Draw a subregion of an image.
///
/// Most often used to draw a sprite from a sprite atlas.
pub fn draw_sub_image(i: &SubImage, p: Point) {
    let ptr = i.raw.as_ptr();
    let len = i.raw.len();
    unsafe {
        b::draw_sub_image(
            ptr as u32,
            len as u32,
            p.x,
            p.y,
            i.point.x,
            i.point.y,
            i.size.width,
            i.size.height,
        );
    }
}
