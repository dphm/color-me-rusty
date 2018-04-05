mod color;

use color::Color;

const RED: Color = Color(255, 0, 0);
const BLUE: Color = Color(0, 0, 255);
const NUM_FRAMES: i32 = 100;

extern {
    fn jsSetBackgroundColor(valsPtr: *const u8);
}

#[no_mangle]
pub extern fn draw_frame(n: i32) {
    let frame = n % (2 * NUM_FRAMES);
    let ratio = if frame < NUM_FRAMES {
        frame as f32 / NUM_FRAMES as f32
    } else {
        1_f32 - (frame % NUM_FRAMES) as f32 / NUM_FRAMES as f32
    };

    let color = color::Gradient::interpolated_color(&RED, &BLUE, ratio);
    let vals = vec![color.0, color.1, color.2];
    unsafe { jsSetBackgroundColor(vals.as_ptr()); }
}

fn main() {}
