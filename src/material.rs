use super::color::*;
use super::object::*;
use super::ray::*;
use super::vec3::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Color),
    Metal(Color),
}

impl Material {
    pub fn scatter(self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        match self {
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

            Material::Metal(albedo) => {
                let reflected = ray.direction.normalize().reflect(hit.normal);

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
        }
    }
}
