#[link(wasm_import_module = "graphics")]
extern "C" {
    pub(crate) fn draw_triangle(
        p1x: i32,
        p1y: i32,
        p2x: i32,
        p2y: i32,
        p3x: i32,
        p3y: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
}
