mod camera;
mod color;
mod constants;
mod material;
mod object;
mod ray;
mod vec3;

use camera::*;
use color::*;
use constants::*;
use material::*;
use object::*;
use rand::prelude::*;
use ray::*;
use vec3::*;

fn main() {
    let look_from = Vec3(3.0, 3.0, 2.0);
    let look_at = Vec3(0.0, 0.0, -1.0);
    let camera = Camera::new(
        Vec3(3.0, 3.0, 2.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        2.0,
        Vec3::length(look_from - look_at),
    );

    let world = World(vec![
        Object::Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertian(Color(Vec3(0.8, 0.8, 0.0))),
        },
        Object::Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Lambertian(Color(Vec3(0.1, 0.2, 0.5))),
        },
        Object::Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Dielectric(1.5),
        },
        Object::Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Material::Dielectric(1.5),
        },
        Object::Sphere {
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Material::Metal(Color(Vec3(0.8, 0.6, 0.2)), 0.0),
        },
    ]);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut color = Color(Vec3(0.0, 0.0, 0.0));
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &world, MAX_DEPTH);
            }

            println!("{}", color);
        }
    }

    eprintln!("Done.");
}

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    if depth < 1 {
        return Color(Vec3(0.0, 0.0, 0.0));
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color(Vec3(0.0, 0.0, 0.0));
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.1 + 1.0);

    Color((1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0))
}
