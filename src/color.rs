use super::constants::*;
use super::vec3::*;
use std::{fmt, ops};

#[derive(Copy, Clone)]
pub struct Color(pub Vec3);

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0 += rhs.0;
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color(self.0 * rhs)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color(self * rhs.0)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = f64::sqrt(self.0 .0 * (1.0 / SAMPLES_PER_PIXEL as f64));
        let g = f64::sqrt(self.0 .1 * (1.0 / SAMPLES_PER_PIXEL as f64));
        let b = f64::sqrt(self.0 .2 * (1.0 / SAMPLES_PER_PIXEL as f64));

        write!(
            f,
            "{} {} {}",
            (256.0 * num::clamp(r, 0.0, 0.999)) as i32,
            (256.0 * num::clamp(g, 0.0, 0.999)) as i32,
            (256.0 * num::clamp(b, 0.0, 0.999)) as i32,
        )
    }
}
