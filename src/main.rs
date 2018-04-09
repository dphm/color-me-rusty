mod color;

use color::Color;

const RED: Color = Color(255, 0, 0);
const BLUE: Color = Color(0, 0, 255);
const NUM_FRAMES: u32 = 100;

extern {
    fn jsSetBackgroundColor(valsPtr: *const u8);
}

#[no_mangle]
pub extern fn draw_frame(n: u32) {
    let color = linear_bounce(&RED, &BLUE, n, NUM_FRAMES);
    let vals = vec![color.0, color.1, color.2];
    unsafe { jsSetBackgroundColor(vals.as_ptr()); }
}

fn linear_bounce(a: &Color, b: &Color, n: u32, num_frames: u32) -> Color {
    let double_frames = 2 * num_frames;
    let frame = n % double_frames;
    if frame < num_frames {
        Color::step(&a, &b, frame, num_frames)
    } else {
        Color::step(&a, &b, double_frames - frame, num_frames)
    }
}

fn main() {}
