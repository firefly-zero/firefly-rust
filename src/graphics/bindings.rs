#[link(wasm_import_module = "graphics")]
extern {
    pub(crate) fn clear_screen(color: i32);
    pub(crate) fn set_color(index: i32, r: i32, g: i32, b: i32);
    pub(crate) fn draw_point(x: i32, y: i32, color: i32);
    pub(crate) fn draw_line(
        p1_x: i32,
        p1_y: i32,
        p2_x: i32,
        p2_y: i32,
        color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_rect(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_rounded_rect(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        corner_width: i32,
        corner_height: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_circle(
        x: i32,
        y: i32,
        diameter: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_ellipse(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_triangle(
        p1_x: i32,
        p1_y: i32,
        p2_x: i32,
        p2_y: i32,
        p3_x: i32,
        p3_y: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_arc(
        x: i32,
        y: i32,
        diameter: i32,
        angle_start: f32,
        angle_sweep: f32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_sector(
        x: i32,
        y: i32,
        diameter: i32,
        angle_start: f32,
        angle_sweep: f32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_text(
        text_ptr: u32,
        text_len: u32,
        font_ptr: u32,
        font_len: u32,
        x: i32,
        y: i32,
        color: i32,
    );
    pub(crate) fn draw_qr(ptr: u32, len: u32, x: i32, y: i32, black: i32, white: i32);
    pub(crate) fn draw_sub_image(
        ptr: u32,
        len: u32,
        x: i32,
        y: i32,
        sub_x: i32,
        sub_y: i32,
        sub_width: i32,
        sub_height: i32,
    );
    pub(crate) fn draw_image(ptr: u32, len: u32, x: i32, y: i32);
    pub(crate) fn set_canvas(ptr: u32, len: u32);
    pub(crate) fn unset_canvas();
}
