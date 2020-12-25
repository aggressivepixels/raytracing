use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
    pub t: f64,
}

#[derive(Clone, Copy)]
pub enum Object {
    Sphere {
        center: Vec3,
        radius: f64,
        material: Material,
    },
}

impl Object {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match *self {
            Object::Sphere {
                center,
                radius,
                material,
            } => {
                let oc = ray.origin - center;
                let a = ray.direction.squared_length();
                let half_b = oc.dot(ray.direction);
                let c = oc.squared_length() - radius.powi(2);
                let discriminant = half_b.powi(2) - (a * c);

                if discriminant < 0.0 {
                    return None;
                }

                let sqrtd = discriminant.sqrt();

                let mut root = (-half_b - sqrtd) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + sqrtd) / a;
                    if root < t_min || t_max < root {
                        return None;
                    }
                }

                let point = ray.at(root);
                let outward_normal = (point - center) / radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;

                Some(Hit {
                    point,
                    normal: if front_face {
                        outward_normal
                    } else {
                        -outward_normal
                    },
                    front_face,
                    material,
                    t: root,
                })
            }
        }
    }
}
