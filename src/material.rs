use super::object::*;
use super::ray::*;
use super::vec3::*;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Lambertian(albedo) => {
                let direction = hit.normal + Vec3::random_in_unit_sphere().normalize();

                Some((
                    albedo,
                    Ray {
                        origin: hit.point,
                        direction: if direction.is_near_zero() {
                            hit.normal
                        } else {
                            direction
                        },
                    },
                ))
            }

            Material::Metal(albedo, fuzz) => {
                let reflected = ray.direction.normalize().reflect(hit.normal)
                    + fuzz * Vec3::random_in_unit_sphere();

                if reflected.dot(hit.normal) > 0.0 {
                    Some((
                        albedo,
                        Ray {
                            origin: hit.point,
                            direction: reflected,
                        },
                    ))
                } else {
                    None
                }
            }

            Material::Dielectric(ir) => {
                let refraction_ratio = if hit.front_face { 1.0 / ir } else { ir };
                let unit_direction = ray.direction.normalize();
                let cos_theta = f64::min(Vec3::dot(-unit_direction, hit.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > random::<f64>()
                {
                    unit_direction.reflect(hit.normal)
                } else {
                    unit_direction.refract(hit.normal, refraction_ratio)
                };

                Some((
                    Vec3(1.0, 1.0, 1.0),
                    Ray {
                        origin: hit.point,
                        direction,
                    },
                ))
            }
        }
    }
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let r = f64::powi((1.0 - ref_idx) / (1.0 + ref_idx), 2);
    return r + (1.0 - r) * f64::powi(1.0 - cos, 5);
}
