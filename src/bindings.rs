#[link(wasm_import_module = "graphics")]
extern {
    pub(crate) fn get_screen_size() -> i32;
    pub(crate) fn clear_screen(color: i32);
    pub(crate) fn set_color(index: i32, r: i32, g: i32, b: i32);
    pub(crate) fn set_colors(
        c1_r: i32,
        c1_g: i32,
        c1_b: i32,
        c2_r: i32,
        c2_g: i32,
        c2_b: i32,
        c3_r: i32,
        c3_g: i32,
        c3_b: i32,
        c4_r: i32,
        c4_g: i32,
        c4_b: i32,
    );
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
        angle_start: i32,
        angle_sweep: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_sector(
        x: i32,
        y: i32,
        diameter: i32,
        angle_start: i32,
        angle_sweep: i32,
        fill_color: i32,
        stroke_color: i32,
        stroke_width: i32,
    );
    pub(crate) fn draw_text(
        text_ptr: i32,
        text_len: i32,
        font_ptr: i32,
        font_len: i32,
        x: i32,
        y: i32,
        color: i32,
    );
    pub(crate) fn draw_sub_image(
        ptr: i32,
        len: i32,
        x: i32,
        y: i32,
        sub_x: i32,
        sub_y: i32,
        sub_width: i32,
        sub_height: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
    );
    pub(crate) fn draw_image(
        ptr: i32,
        len: i32,
        x: i32,
        y: i32,
        c1: i32,
        c2: i32,
        c3: i32,
        c4: i32,
    );
}

#[link(wasm_import_module = "fs")]
extern {
    pub(crate) fn get_rom_file_size(path_ptr: u32, path_len: u32) -> u32;
    // pub(crate) fn get_file_size(path_ptr: u32, path_len: u32) -> u32;
    pub(crate) fn load_rom_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
    // pub(crate) fn load_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
    // pub(crate) fn dump_file(path_ptr: u32, path_len: u32, buf_ptr: u32, buf_len: u32) -> u32;
    // pub(crate) fn remove_file(path_ptr: u32, path_len: u32);
}

#[link(wasm_import_module = "input")]
extern {
    pub(crate) fn read_pad() -> u32;
    pub(crate) fn read_buttons() -> u32;
}
