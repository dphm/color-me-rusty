#[derive(Clone, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
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

    #[test]
    fn zero_steps() {
        let black = Color(0, 0, 0);
        let white = Color(255, 255, 255);
        assert_eq!(black, Color::step(&black, &white, 0, 0));
    }

    #[test]
    fn one_step() {
        let black = Color(0, 0, 0);
        let white = Color(255, 255, 255);
        assert_eq!(black, Color::step(&black, &white, 0, 1));
        assert_eq!(white, Color::step(&black, &white, 1, 1));
    }

    #[test]
    fn two_steps() {
        let black = Color(0, 0, 0);
        let half = Color(127, 127, 127);
        let white = Color(255, 255, 255);
        assert_eq!(black, Color::step(&black, &white, 0, 2));
        assert_eq!(half, Color::step(&black, &white, 1, 2));
        assert_eq!(white, Color::step(&black, &white, 2, 2));
    }

    #[test]
    fn three_steps() {
        let black = Color(0, 0, 0);
        let one_third = Color(85, 85, 85);
        let two_thirds = Color(170, 170, 170);
        let white = Color(255, 255, 255);
        assert_eq!(black, Color::step(&black, &white, 0, 3));
        assert_eq!(one_third, Color::step(&black, &white, 1, 3));
        assert_eq!(two_thirds, Color::step(&black, &white, 2, 3));
        assert_eq!(white, Color::step(&black, &white, 3, 3));
    }

    #[test]
    fn zero_steps_down() {
        let black = Color(0, 0, 0);
        let white = Color(255, 255, 255);
        assert_eq!(white, Color::step(&white, &black, 0, 0));
    }

    #[test]
    fn one_step_down() {
        let black = Color(0, 0, 0);
        let white = Color(255, 255, 255);
        assert_eq!(white, Color::step(&white, &black, 0, 1));
        assert_eq!(black, Color::step(&white, &black, 1, 1));
    }

    #[test]
    fn two_steps_down() {
        let black = Color(0, 0, 0);
        let half = Color(127, 127, 127);
        let white = Color(255, 255, 255);
        assert_eq!(white, Color::step(&white, &black, 0, 2));
        assert_eq!(half, Color::step(&white, &black, 1, 2));
        assert_eq!(black, Color::step(&white, &black, 2, 2));
    }

    #[test]
    fn three_steps_down() {
        let black = Color(0, 0, 0);
        let one_third = Color(85, 85, 85);
        let two_thirds = Color(170, 170, 170);
        let white = Color(255, 255, 255);
        assert_eq!(white, Color::step(&white, &black, 0, 3));
        assert_eq!(two_thirds, Color::step(&white, &black, 1, 3));
        assert_eq!(one_third, Color::step(&white, &black, 2, 3));
        assert_eq!(black, Color::step(&white, &black, 3, 3));
    }

    #[test]
    fn two_steps_unequal_values() {
        let red = Color(255, 0, 0);
        let mid = Color(127, 0, 127);
        let blue = Color(0, 0, 255);
        assert_eq!(red, Color::step(&red, &blue, 0, 2));
        assert_eq!(mid, Color::step(&red, &blue, 1, 2));
        assert_eq!(blue, Color::step(&red, &blue, 2, 2));
    }
}
