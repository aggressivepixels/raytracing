mod camera;
mod constants;
mod material;
mod object;
mod ray;
mod vec3;

use camera::*;
use constants::*;
use material::*;
use object::*;
use rand::prelude::*;
use ray::*;
use vec3::*;

fn main() {
    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    let scene = random_scene();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &scene, MAX_DEPTH);
            }

            print_color(&color);
        }
    }

    eprintln!("Done.");
}

fn ray_color(ray: &Ray, scene: &[Object], depth: i32) -> Vec3 {
    if depth < 1 {
        return Vec3(0.0, 0.0, 0.0);
    }

    if let Some(hit) = hit_scene(scene, ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, scene, depth - 1);
        }

        return Vec3(0.0, 0.0, 0.0);
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.1 + 1.0);

    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

pub fn hit_scene(scene: &[Object], ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut closest_so_far = t_max;
    let mut hit: Option<Hit> = None;

    for &obj in scene {
        if let Some(new_hit) = obj.hit(ray, t_min, closest_so_far) {
            closest_so_far = new_hit.t;
            hit = Some(new_hit);
        }
    }

    hit
}

fn random_scene() -> Vec<Object> {
    let mut rng = rand::thread_rng();
    let mut objects = vec![];

    objects.push(Object::Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Vec3(0.5, 0.5, 0.5)),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if Vec3::length(center - Vec3(4.0, 0.2, 0.0)) > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3(random::<f64>(), random::<f64>(), random::<f64>())
                        * Vec3(random::<f64>(), random::<f64>(), random::<f64>());

                    objects.push(Object::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian(albedo),
                    });
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_in_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);

                    objects.push(Object::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(albedo, fuzz),
                    });
                } else {
                    objects.push(Object::Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    });
                }
            }
        }
    }

    objects.push(Object::Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    });

    objects.push(Object::Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Vec3(0.4, 0.2, 0.1)),
    });

    objects.push(Object::Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Vec3(0.7, 0.6, 0.5), 0.0),
    });

    objects
}

fn print_color(v: &Vec3) {
    let r = f64::sqrt(v.0 * (1.0 / SAMPLES_PER_PIXEL as f64));
    let g = f64::sqrt(v.1 * (1.0 / SAMPLES_PER_PIXEL as f64));
    let b = f64::sqrt(v.2 * (1.0 / SAMPLES_PER_PIXEL as f64));

    println!(
        "{} {} {}",
        (256.0 * num::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * num::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * num::clamp(b, 0.0, 0.999)) as i32,
    )
}
