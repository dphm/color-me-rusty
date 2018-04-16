#[derive(Clone, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn interpolate_linear(colors: &[&Color], n: u32, num_frames: u32) -> Color {
        let num_colors = colors.len() as u32;
        let end = num_frames * num_colors;
        let frame = n % end;

        let a_i = (frame / num_frames) as usize;
        let b_i = (a_i + 1) % num_colors as usize;

        Color::step(colors[a_i], colors[b_i], n % num_frames, num_frames)
    }

    pub fn step(from: &Color, to: &Color, step: u32, num_steps: u32) -> Color {
        if num_steps == 0 { return from.clone(); }
        let ratio = step as f32 / num_steps as f32;
        Color(
            Color::val_between(from.0, to.0, ratio),
            Color::val_between(from.1, to.1, ratio),
            Color::val_between(from.2, to.2, ratio)
        )
    }

    fn val_between(from: u8, to: u8, ratio: f32) -> u8 {
        let (min, max) = if from < to { (from, to) } else { (to, from) };
        let diff = max - min;
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

    const BLACK: Color = Color(0, 0, 0);
    const WHITE: Color = Color(255, 255, 255);
    const RED: Color = Color(255, 0, 0);
    const BLUE: Color = Color(0, 0, 255);

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
        let half = Color(127, 127, 127);
        assert_eq!(BLACK, Color::step(&BLACK, &WHITE, 0, 2));
        assert_eq!(half, Color::step(&BLACK, &WHITE, 1, 2));
        assert_eq!(WHITE, Color::step(&BLACK, &WHITE, 2, 2));
    }

    #[test]
    fn three_steps() {
        let one_third = Color(85, 85, 85);
        let two_thirds = Color(170, 170, 170);
        assert_eq!(BLACK, Color::step(&BLACK, &WHITE, 0, 3));
        assert_eq!(one_third, Color::step(&BLACK, &WHITE, 1, 3));
        assert_eq!(two_thirds, Color::step(&BLACK, &WHITE, 2, 3));
        assert_eq!(WHITE, Color::step(&BLACK, &WHITE, 3, 3));
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
        let half = Color(127, 127, 127);
        assert_eq!(WHITE, Color::step(&WHITE, &BLACK, 0, 2));
        assert_eq!(half, Color::step(&WHITE, &BLACK, 1, 2));
        assert_eq!(BLACK, Color::step(&WHITE, &BLACK, 2, 2));
    }

    #[test]
    fn three_steps_down() {
        let one_third = Color(85, 85, 85);
        let two_thirds = Color(170, 170, 170);
        assert_eq!(WHITE, Color::step(&WHITE, &BLACK, 0, 3));
        assert_eq!(two_thirds, Color::step(&WHITE, &BLACK, 1, 3));
        assert_eq!(one_third, Color::step(&WHITE, &BLACK, 2, 3));
        assert_eq!(BLACK, Color::step(&WHITE, &BLACK, 3, 3));
    }

    #[test]
    fn two_steps_unequal_values() {
        let mid = Color(127, 0, 127);
        assert_eq!(RED, Color::step(&RED, &BLUE, 0, 2));
        assert_eq!(mid, Color::step(&RED, &BLUE, 1, 2));
        assert_eq!(BLUE, Color::step(&RED, &BLUE, 2, 2));
    }
}
