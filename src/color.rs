use super::vec3::*;
use std::fmt;

pub struct Color(pub Vec3);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.0 .0 * 255.999) as i32,
            (self.0 .1 * 255.999) as i32,
            (self.0 .2 * 255.999) as i32
        )
    }
}
