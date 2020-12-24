mod color;
mod ray;
mod vec3;

use color::*;
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

            println!("{}", ray_color(&ray));
        }
    }

    eprintln!("Done.");
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, ray) {
        return Color(Vec3(1.0, 0.0, 0.0));
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.1 + 1.0);

    Color((1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0))
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction.squared_length();
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.squared_length() - radius.powi(2);
    let discriminant = b.powi(2) - (4.0 * a * c);

    discriminant > 0.0
}
