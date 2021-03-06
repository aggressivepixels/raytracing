mod camera;
mod material;
mod object;
mod ray;
mod vec3;

use camera::Camera;
use material::Material::{Dielectric, Lambertian, Metal};
use object::Hit;
use object::Object::{self, Sphere};
use rand::{Rng, SeedableRng};
use ray::Ray;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 2560;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const SAMPLES_PER_PIXEL: usize = 500;
const MAX_DEPTH: i32 = 50;

const LOOKFROM: Vec3 = Vec3(13.0, 2.0, 3.0);
const LOOKAT: Vec3 = Vec3(0.0, 0.0, 0.0);
const VUP: Vec3 = Vec3(0.0, 1.0, 0.0);
const VFOV: f64 = 20.0;
const APERTURE: f64 = 0.1;
const FOCUS_DIST: f64 = 10.0;

const BACKGROUND_GRADIENT_START: Vec3 = Vec3(0.5, 0.7, 1.0);
const BACKGROUND_GRADIENT_END: Vec3 = Vec3(1.0, 1.0, 1.0);

fn main() {
    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    let camera = Camera::new(
        LOOKFROM,
        LOOKAT,
        VUP,
        VFOV,
        ASPECT_RATIO,
        APERTURE,
        FOCUS_DIST,
    );

    let scene = random_scene(&mut rng);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for y in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", y + 1);

        for x in 0..IMAGE_WIDTH {
            print_color(&pixel_color(x, y, &camera, &scene, &mut rng));
        }
    }

    eprintln!("Done.");
}

fn pixel_color<R: Rng + ?Sized>(
    x: usize,
    y: usize,
    camera: &Camera,
    scene: &[Object],
    rng: &mut R,
) -> Vec3 {
    (0..SAMPLES_PER_PIXEL).fold(Vec3::ZERO, |acc, _| {
        let u = (x as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
        let v = (y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
        let ray = camera.get_ray(u, v, rng);

        acc + ray_color(&ray, &scene, MAX_DEPTH, rng)
    }) / SAMPLES_PER_PIXEL as f64
}

fn ray_color<R: Rng + ?Sized>(ray: &Ray, scene: &[Object], depth: i32, rng: &mut R) -> Vec3 {
    if depth < 1 {
        return Vec3::ZERO;
    }

    if let Some(hit) = hit_scene(scene, ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit, rng) {
            return attenuation * ray_color(&scattered, scene, depth - 1, rng);
        }

        return Vec3::ZERO;
    }

    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.1 + 1.0);

    (1.0 - t) * BACKGROUND_GRADIENT_END + t * BACKGROUND_GRADIENT_START
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

fn random_scene<R: Rng + ?Sized>(rng: &mut R) -> Vec<Object> {
    let mut objects = vec![];

    objects.push(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian(Vec3(0.5, 0.5, 0.5)),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if Vec3::length(&(center - Vec3(4.0, 0.2, 0.0))) > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
                        * Vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());

                    objects.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian(albedo),
                    });
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_in_range(0.5, 1.0, rng);
                    let fuzz = rng.gen_range(0.0..0.5);

                    objects.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal(albedo, fuzz),
                    });
                } else {
                    objects.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric(1.5),
                    });
                }
            }
        }
    }

    objects.push(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric(1.5),
    });

    objects.push(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian(Vec3(0.4, 0.2, 0.1)),
    });

    objects.push(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal(Vec3(0.7, 0.6, 0.5), 0.0),
    });

    objects
}

fn print_color(v: &Vec3) {
    println!(
        "{} {} {}",
        (256.0 * num::clamp(f64::sqrt(v.0), 0.0, 0.999)) as i32,
        (256.0 * num::clamp(f64::sqrt(v.1), 0.0, 0.999)) as i32,
        (256.0 * num::clamp(f64::sqrt(v.2), 0.0, 0.999)) as i32,
    )
}
