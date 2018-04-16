#![feature(range_contains)]

mod color;

use color::Color;

const RED: Color = Color(255, 0, 0);
const GREEN: Color = Color(0, 255, 0);
const BLUE: Color = Color(0, 0, 255);
const NUM_FRAMES: u32 = 200;

extern {
    fn jsSetBackgroundColor(valsPtr: *const u8);
}

#[no_mangle]
pub extern fn draw_frame(n: u32) {
    let colors = vec![&RED, &GREEN, &BLUE];
    let color = Color::interpolate_linear(&colors, n, NUM_FRAMES);
    let vals = vec![color.0, color.1, color.2];
    unsafe { jsSetBackgroundColor(vals.as_ptr()); }
}

fn main() {}
