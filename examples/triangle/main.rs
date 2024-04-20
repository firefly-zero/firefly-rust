#![no_std]
#![no_main]
use firefly_rust as ff;

#[no_mangle]
extern fn boot() {
    ff::draw_triangle(
        ff::Point { x: 60, y: 10 },
        ff::Point { x: 40, y: 40 },
        ff::Point { x: 80, y: 40 },
        ff::Style {
            fill_color:   ff::Color::ACCENT,
            stroke_color: ff::Color::SECONDARY,
            stroke_width: 1,
        },
    );
}
