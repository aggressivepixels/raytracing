use super::ray::*;
use super::vec3::*;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::normalize(look_from - look_at);
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left = look_from - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + s * self.horizontal + t * self.vertical - self.origin,
        }
    }
}
