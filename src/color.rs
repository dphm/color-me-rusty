#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    red:   u8,
    green: u8,
    blue:  u8,
}

impl Color {
    pub const RED:   Color = Color { red: 255, green:   0, blue:   0 };
    pub const GREEN: Color = Color { red:   0, green: 255, blue:   0 };
    pub const BLUE:  Color = Color { red:   0, green:   0, blue: 255 };

    /// Interpolates between every adjacent pair of colors in `frames_per_color` frames.
    ///
    /// It takes `frames_per_color` frames to interpolate from one color to the next.
    /// The last color interpolates to the first color in `frames_per_color` frames.
    ///
    /// # Panics
    ///
    /// Panics when no colors are given.
    pub fn interpolate_linear(colors: &[&Color], frame: usize, frames_per_color: usize) -> Color {
        let num_colors = colors.len();
        if num_colors == 0 { panic!("expected colors"); }
        if num_colors == 1 || frames_per_color == 0 { return colors[0].clone(); }

        let from = frame / frames_per_color % num_colors;
        let to   = (from + 1) % num_colors;
        Color::step(colors[from], colors[to], frame % frames_per_color, frames_per_color)
    }

    /// Returns the color with values `frame` of `frames_per_color` between `from` and `to`.
    pub fn step(from: &Color, to: &Color, frame: usize, frames_per_color: usize) -> Color {
        if frames_per_color == 0 { return from.clone(); }
        let ratio = frame as f32 / frames_per_color as f32;
        Color {
            red:   Color::val_between(  from.red, to.red,   ratio),
            green: Color::val_between(from.green, to.green, ratio),
            blue:  Color::val_between( from.blue, to.blue,  ratio),
        }
    }

    /// Returns values as a Vec<u8> of length 3.
    pub fn values(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }

    /// Returns the value between `from` and `to`.
    ///
    /// A ratio of 0.0 is `from`.
    /// A ratio of 0.5 is half way between `from` and `to`.
    /// A ratio of 1.0 is `to`.
    fn val_between(from: u8, to: u8, ratio: f32) -> u8 {
        let (min, max) = if from < to { (from, to) } else { (to, from) };
        let diff  = max - min;
        let delta = ratio * (diff as f32);
        if from < to {
            from.wrapping_add(delta as u8)
        } else {
            from.wrapping_sub(delta.ceil() as u8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: Color = Color { red:   0, green:   0, blue:   0 };
    const WHITE: Color = Color { red: 255, green: 255, blue: 255 };
    const RED:   Color = Color { red: 255, green:   0, blue:   0 };
    const GREEN: Color = Color { red:   0, green: 255, blue:   0 };
    const BLUE:  Color = Color { red:   0, green:   0, blue: 255 };

    #[test]
    fn zero_steps() {
        assert_eq!(BLACK, Color::step(&BLACK, &WHITE, 0, 0));
    }

    #[test]
    fn one_step() {
        assert_eq!(BLACK, Color::step(&BLACK, &WHITE, 0, 1));
        assert_eq!(WHITE, Color::step(&BLACK, &WHITE, 1, 1));
    }

    #[test]
    fn two_steps() {
        let half = Color { red: 127, green: 127, blue: 127 };
        assert_eq!(BLACK, Color::step(&BLACK, &WHITE, 0, 2));
        assert_eq!(half,  Color::step(&BLACK, &WHITE, 1, 2));
        assert_eq!(WHITE, Color::step(&BLACK, &WHITE, 2, 2));
    }

    #[test]
    fn three_steps() {
        let one_third  = Color { red:  85, green:  85, blue:  85 };
        let two_thirds = Color { red: 170, green: 170, blue: 170 };
        assert_eq!(BLACK,      Color::step(&BLACK, &WHITE, 0, 3));
        assert_eq!(one_third,  Color::step(&BLACK, &WHITE, 1, 3));
        assert_eq!(two_thirds, Color::step(&BLACK, &WHITE, 2, 3));
        assert_eq!(WHITE,      Color::step(&BLACK, &WHITE, 3, 3));
    }

    #[test]
    fn zero_steps_down() {
        assert_eq!(WHITE, Color::step(&WHITE, &BLACK, 0, 0));
    }

    #[test]
    fn one_step_down() {
        assert_eq!(WHITE, Color::step(&WHITE, &BLACK, 0, 1));
        assert_eq!(BLACK, Color::step(&WHITE, &BLACK, 1, 1));
    }

    #[test]
    fn two_steps_down() {
        let half = Color { red: 127, green: 127, blue: 127 };
        assert_eq!(WHITE, Color::step(&WHITE, &BLACK, 0, 2));
        assert_eq!(half,  Color::step(&WHITE, &BLACK, 1, 2));
        assert_eq!(BLACK, Color::step(&WHITE, &BLACK, 2, 2));
    }

    #[test]
    fn three_steps_down() {
        let one_third  = Color { red:  85, green:  85, blue:  85 };
        let two_thirds = Color { red: 170, green: 170, blue: 170 };
        assert_eq!(WHITE,      Color::step(&WHITE, &BLACK, 0, 3));
        assert_eq!(two_thirds, Color::step(&WHITE, &BLACK, 1, 3));
        assert_eq!(one_third,  Color::step(&WHITE, &BLACK, 2, 3));
        assert_eq!(BLACK,      Color::step(&WHITE, &BLACK, 3, 3));
    }

    #[test]
    fn two_steps_unequal_values() {
        let mid = Color { red: 127, green: 0, blue: 127 };
        assert_eq!(RED,  Color::step(&RED, &BLUE, 0, 2));
        assert_eq!(mid,  Color::step(&RED, &BLUE, 1, 2));
        assert_eq!(BLUE, Color::step(&RED, &BLUE, 2, 2));
    }

    #[should_panic]
    #[test]
    fn interpolate_linear_no_colors() {
        let colors = vec![];
        Color::interpolate_linear(&colors, 0, 0);
    }

    #[test]
    fn interpolate_linear_one_color() {
        let colors = vec![&RED];
        assert_eq!(RED, Color::interpolate_linear(&colors, 0, 1));
        assert_eq!(RED, Color::interpolate_linear(&colors, 1, 1));
        assert_eq!(RED, Color::interpolate_linear(&colors, 2, 1));
    }

    #[test]
    fn interpolate_linear_two_colors() {
        let colors = vec![&RED, &GREEN];
        let mid = Color { red: 127, green: 127, blue: 0 };
        assert_eq!(RED,   Color::interpolate_linear(&colors, 0, 2));
        assert_eq!(mid,   Color::interpolate_linear(&colors, 1, 2));
        assert_eq!(GREEN, Color::interpolate_linear(&colors, 2, 2));
        assert_eq!(mid,   Color::interpolate_linear(&colors, 3, 2));
        assert_eq!(RED,   Color::interpolate_linear(&colors, 4, 2));
    }

    #[test]
    fn interpolate_linear_three_colors() {
        let colors = vec![&RED, &GREEN, &BLUE];
        let mid_rg = Color { red: 127, green: 127, blue:   0 };
        let mid_gb = Color { red:   0, green: 127, blue: 127 };
        let mid_br = Color { red: 127, green:   0, blue: 127 };
        assert_eq!(RED,    Color::interpolate_linear(&colors, 0, 2));
        assert_eq!(mid_rg, Color::interpolate_linear(&colors, 1, 2));
        assert_eq!(GREEN,  Color::interpolate_linear(&colors, 2, 2));
        assert_eq!(mid_gb, Color::interpolate_linear(&colors, 3, 2));
        assert_eq!(BLUE,   Color::interpolate_linear(&colors, 4, 2));
        assert_eq!(mid_br, Color::interpolate_linear(&colors, 5, 2));
        assert_eq!(RED,    Color::interpolate_linear(&colors, 6, 2));
    }

    #[test]
    fn interpolate_linear_three_colors_many_steps() {
        let colors = vec![&RED, &GREEN, &BLUE];
        let mid_rg = Color { red: 127, green: 127, blue:   0 };
        let mid_gb = Color { red:   0, green: 127, blue: 127 };
        let mid_br = Color { red: 127, green:   0, blue: 127 };
        assert_eq!(RED,    Color::interpolate_linear(&colors,   0, 200));
        assert_eq!(mid_rg, Color::interpolate_linear(&colors, 100, 200));
        assert_eq!(GREEN,  Color::interpolate_linear(&colors, 200, 200));
        assert_eq!(mid_gb, Color::interpolate_linear(&colors, 300, 200));
        assert_eq!(BLUE,   Color::interpolate_linear(&colors, 400, 200));
        assert_eq!(mid_br, Color::interpolate_linear(&colors, 500, 200));
        assert_eq!(RED,    Color::interpolate_linear(&colors, 600, 200));
    }

    #[test]
    fn values_vec() {
        assert_eq!(vec![  0,   0,   0], BLACK.values());
        assert_eq!(vec![255, 255, 255], WHITE.values());
        assert_eq!(vec![255,   0,   0], RED.values());
        assert_eq!(vec![  0, 255,   0], GREEN.values());
        assert_eq!(vec![  0,   0, 255], BLUE.values());
    }
}
