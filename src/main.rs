mod camera;
mod color;
mod constants;
mod object;
mod ray;
mod vec3;

use camera::*;
use color::*;
use constants::*;
use object::*;
use rand::prelude::*;
use ray::*;
use vec3::*;

fn main() {
    let camera = Camera::new(ASPECT_RATIO);
    let world = World(vec![
        Object::Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Object::Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
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

                color += ray_color(&ray, &world)
            }

            println!("{}", color);
        }
    }

    eprintln!("Done.");
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        return Color(0.5 * (hit.normal + Vec3(1.0, 1.0, 1.0)));
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.1 + 1.0);

    Color((1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0))
}
