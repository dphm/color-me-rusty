#[derive(Clone, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

pub struct Gradient {
    pub colors: Vec<Color>
}

impl Gradient {
    pub fn interpolate(a: &Color, b: &Color, steps: usize) -> Gradient {
        let mut colors = Vec::new();
        for step in 0..steps + 1 {
            let ratio = step as f32 / steps as f32;
            let step_color = Gradient::interpolated_color(a, b, ratio);
            colors.push(step_color);
        }

        Gradient { colors }
    }

    pub fn interpolated_color(a: &Color, b: &Color, ratio: f32) -> Color {
        Color(
            Gradient::between(a.0, b.0, ratio),
            Gradient::between(a.1, b.1, ratio),
            Gradient::between(a.2, b.2, ratio)
        )
    }

    fn between(a: u8, b: u8, ratio: f32) -> u8 {
        let (min, max) = if a < b { (a, b) } else { (b, a) };
        let diff = max - min;
        let delta = ratio * (diff as f32);
        if a < b {
            a.wrapping_add(delta as u8)
        } else {
            a.wrapping_sub(delta.ceil() as u8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_one_step() {
        let black = Color(0, 0, 0);
        let white = Color(255, 255, 255);
        let gradient = Gradient::interpolate(&black, &white, 1);
        assert_eq!(vec![black, white], gradient.colors);
    }

    #[test]
    fn interpolate_two_steps() {
        let black = Color(0, 0, 0);
        let half = Color(127, 127, 127);
        let white = Color(255, 255, 255);
        let gradient = Gradient::interpolate(&black, &white, 2);
        assert_eq!(vec![black, half, white], gradient.colors);
    }

    #[test]
    fn interpolate_three_steps() {
        let black = Color(0, 0, 0);
        let third_1 = Color(85, 85, 85);
        let third_2 = Color(170, 170, 170);
        let white = Color(255, 255, 255);
        let gradient = Gradient::interpolate(&black, &white, 3);
        assert_eq!(vec![black, third_1, third_2, white], gradient.colors);
    }

    #[test]
    fn interpolate_one_step_reverse() {
        let black = Color(0, 0, 0);
        let white = Color(255, 255, 255);
        let gradient = Gradient::interpolate(&white, &black, 1);
        assert_eq!(vec![white, black], gradient.colors);
    }

    #[test]
    fn interpolate_two_steps_reverse() {
        let black = Color(0, 0, 0);
        let half = Color(127, 127, 127);
        let white = Color(255, 255, 255);
        let gradient = Gradient::interpolate(&white, &black, 2);
        assert_eq!(vec![white, half, black], gradient.colors);
    }

    #[test]
    fn interpolate_three_steps_reverse() {
        let black = Color(0, 0, 0);
        let third_1 = Color(85, 85, 85);
        let third_2 = Color(170, 170, 170);
        let white = Color(255, 255, 255);
        let gradient = Gradient::interpolate(&white, &black, 3);
        assert_eq!(vec![white, third_2, third_1, black], gradient.colors);
    }

    #[test]
    fn interpolate_different_values() {
        let red = Color(255, 0, 0);
        let mid = Color(127, 0, 127);
        let blue = Color(0, 0, 255);
        let gradient = Gradient::interpolate(&red, &blue, 2);
        assert_eq!(vec![red, mid, blue], gradient.colors);
    }
}
