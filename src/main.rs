mod color;
mod vec3;

use color::*;
use vec3::*;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let pixel = Vec3(
                i as f64 / (IMAGE_WIDTH as f64 - 1.0),
                j as f64 / (IMAGE_HEIGHT as f64 - 1.0),
                0.25,
            );

            println!("{}", Color(pixel));
        }
    }

    eprintln!("Done.");
}
