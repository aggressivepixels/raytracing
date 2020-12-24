use super::ray::*;
use super::vec3::*;

#[derive(Clone, Copy)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
}

#[derive(Clone, Copy)]
pub enum Object {
    Sphere { center: Vec3, radius: f64 },
}

impl Object {
    pub fn hit(self, ray: &Ray, t_min: f64, t_max: f64, hit: Hit) -> (bool, Hit) {
        match self {
            Object::Sphere { center, radius } => {
                let oc = ray.origin - center;
                let a = ray.direction.squared_length();
                let half_b = oc.dot(ray.direction);
                let c = oc.squared_length() - radius.powi(2);
                let discriminant = half_b.powi(2) - (a * c);

                if discriminant < 0.0 {
                    return (false, hit);
                }

                let sqrtd = discriminant.sqrt();

                let mut root = (-half_b - sqrtd) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + sqrtd) / a;
                    if root < t_min || t_max < root {
                        return (false, hit);
                    }
                }

                let point = ray.at(root);
                let outward_normal = (point - center) / radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;

                (
                    true,
                    Hit {
                        point,
                        normal: if front_face {
                            outward_normal
                        } else {
                            -outward_normal
                        },
                        front_face,
                        t: root,
                    },
                )
            }
        }
    }
}

pub struct World(pub Vec<Object>);

impl World {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, Hit) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut hit = Hit {
            point: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            front_face: false,
            t: 0.0,
        };

        for obj in &self.0 {
            let (did_hit, new_hit) = obj.hit(ray, t_min, closest_so_far, hit);
            if did_hit {
                hit_anything = true;
                closest_so_far = new_hit.t;
                hit = new_hit;
            }
        }

        (hit_anything, hit)
    }
}
