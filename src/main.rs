mod color;

use color::Color;

extern {
    fn jsSetBackgroundColor(valsPtr: *const u8);
}

#[no_mangle]
pub extern fn set_background_color(step: u32) {
    let colors = vec![&Color::RED, &Color::GREEN, &Color::BLUE];
    let color  = Color::interpolate_linear(&colors, step, 200);
    unsafe { jsSetBackgroundColor(color.values().as_ptr()); }
}

fn main() {}
