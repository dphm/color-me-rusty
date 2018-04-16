#[derive(Clone, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn interpolate_linear(colors: &[&Color], step: u32, num_steps: u32) -> Color {
        let num_colors = colors.len() as u32;
        if num_colors == 0 { panic!("expected colors"); }
        if num_colors == 1 || num_steps == 0 { return colors[0].clone(); }

        // Extend steps to interpolate from last to first color
        let steps_per_color = num_steps / (num_colors - 1);
        let end = num_steps + steps_per_color;
        let from = ((step % end) / steps_per_color) as usize;
        let to = (from + 1) % num_colors as usize;
        Color::step(colors[from], colors[to], step % steps_per_color, steps_per_color)
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
    const GREEN: Color = Color(0, 255, 0);
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
        let mid = Color(127, 127, 0);
        assert_eq!(RED, Color::interpolate_linear(&colors, 0, 2));
        assert_eq!(mid, Color::interpolate_linear(&colors, 1, 2));
        assert_eq!(GREEN, Color::interpolate_linear(&colors, 2, 2));
        assert_eq!(mid, Color::interpolate_linear(&colors, 3, 2));
        assert_eq!(RED, Color::interpolate_linear(&colors, 4, 2));
    }

    #[test]
    fn interpolate_linear_three_colors() {
        let colors = vec![&RED, &GREEN, &BLUE];
        let mid_rg = Color(127, 127, 0);
        let mid_gb = Color(0, 127, 127);
        let mid_br = Color(127, 0, 127);
        assert_eq!(RED, Color::interpolate_linear(&colors, 0, 4));
        assert_eq!(mid_rg, Color::interpolate_linear(&colors, 1, 4));
        assert_eq!(GREEN, Color::interpolate_linear(&colors, 2, 4));
        assert_eq!(mid_gb, Color::interpolate_linear(&colors, 3, 4));
        assert_eq!(BLUE, Color::interpolate_linear(&colors, 4, 4));
        assert_eq!(mid_br, Color::interpolate_linear(&colors, 5, 4));
        assert_eq!(RED, Color::interpolate_linear(&colors, 6, 4));
    }

    #[test]
    fn interpolate_linear_three_colors_many_steps() {
        let colors = vec![&RED, &GREEN, &BLUE];
        let mid_rg = Color(127, 127, 0);
        let mid_gb = Color(0, 127, 127);
        let mid_br = Color(127, 0, 127);
        assert_eq!(RED, Color::interpolate_linear(&colors, 0, 400));
        assert_eq!(mid_rg, Color::interpolate_linear(&colors, 100, 400));
        assert_eq!(GREEN, Color::interpolate_linear(&colors, 200, 400));
        assert_eq!(mid_gb, Color::interpolate_linear(&colors, 300, 400));
        assert_eq!(BLUE, Color::interpolate_linear(&colors, 400, 400));
        assert_eq!(mid_br, Color::interpolate_linear(&colors, 500, 400));
        assert_eq!(RED, Color::interpolate_linear(&colors, 600, 400));
    }
}
