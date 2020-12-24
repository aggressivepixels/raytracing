const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.25;

            println!(
                "{} {} {}",
                (r * 255.999) as usize,
                (g * 255.999) as usize,
                (b * 255.999) as usize
            )
        }
    }

    eprintln!("Done.");
}
