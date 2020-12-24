mod color;
mod object;
mod ray;
mod vec3;

use color::*;
use object::*;
use ray::*;
use vec3::*;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);
    const HORIZONTAL: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);

    let lower_left = ORIGIN - (HORIZONTAL / 2.0) - (VERTICAL / 2.0) - Vec3(0.0, 0.0, FOCAL_LENGTH);

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
            let u = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = Ray {
                origin: ORIGIN,
                direction: lower_left + (u * HORIZONTAL) + (v * VERTICAL) - ORIGIN,
            };

            println!("{}", ray_color(&ray, &world));
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
