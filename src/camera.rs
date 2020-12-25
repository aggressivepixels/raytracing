use super::ray::Ray;
use super::vec3::Vec3;
use rand::Rng;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::normalize(look_from - look_at);
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray<R: Rng + ?Sized>(&self, s: f64, t: f64, rng: &mut R) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.0 + self.v * rd.1;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
