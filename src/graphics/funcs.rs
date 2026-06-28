use super::{bindings as b, *};

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

/// Set a single point (1 pixel if scaling is 1) on the frame.
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
/// Unlike in the other drawing functions, here [`Point`] points not to the top-left
/// corner but to the baseline start position ([`Font::baseline`]).
pub fn draw_text<F: Font>(t: &str, f: &F, p: Point, c: Color) {
    let text_ptr = t.as_ptr();
    let text_len = t.len();
    let font = unsafe { f.as_bytes() };
    let font_ptr = font.as_ptr();
    let font_len = font.len();
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

/// Render a QR code for the given text.
pub fn draw_qr(t: &str, p: Point, black: Color, white: Color) {
    let ptr = t.as_ptr();
    let len = t.len();
    unsafe {
        b::draw_qr(ptr as u32, len as u32, p.x, p.y, black.into(), white.into());
    }
}

/// Render an image using the given colors.
pub fn draw_image<I: Image>(i: &I, p: Point) {
    let raw = unsafe { i.as_bytes() };
    let ptr = raw.as_ptr();
    let len = raw.len();
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

/// Tile the given screen area with the provided sub-image.
pub fn draw_sub_tile(i: &SubImage, p: Point, s: Size) {
    let ptr = i.raw.as_ptr();
    let len = i.raw.len();
    unsafe {
        b::draw_sub_tile(
            ptr as u32,
            len as u32,
            p.x,
            p.y,
            s.width,
            s.height,
            i.point.x,
            i.point.y,
            i.size.width,
            i.size.height,
        );
    }
}

/// Fill the given area with the given 9-slice.
///
/// A 9-slice is used to tile an area with 9 sub-images: 4 corners,
/// 4 edges, and 1 middle segment. It is useful for speech bubbles
/// and other stylish boxes.
///
/// The whole image is the 9-slice. The sub-image is the center area of the 9-slice.
///
/// If the target area is bigger than the 9-slice segments,
/// all the segments (except corners) are repeated ("tiled")
/// without stretching or mirroring.
pub fn draw_nine_slice(i: &SubImage, p: Point, s: Size) {
    let ptr = i.raw.as_ptr();
    let len = i.raw.len();
    unsafe {
        b::draw_nine_slice(
            ptr as u32,
            len as u32,
            p.x,
            p.y,
            s.width,
            s.height,
            i.point.x,
            i.point.y,
            i.size.width,
            i.size.height,
        );
    }
}

/// Set canvas to be used for all subsequent drawing operations.
pub fn set_canvas<C: Canvas>(c: &C) {
    let raw = unsafe { c.as_bytes() };
    let ptr = raw.as_ptr();
    let len = raw.len();
    unsafe {
        b::set_canvas(ptr as u32, len as u32);
    }
}

/// Unset canvas set by [`set_canvas`]. All subsequent drawing operations will target frame buffer.
pub fn unset_canvas() {
    unsafe {
        b::unset_canvas();
    }
}
