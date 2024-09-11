use std::io;

use color::Color;

mod color;
mod rays;
mod vec3;

fn main() {
    // Image dimensions
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in (0..IMAGE_WIDTH) {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let b = 0.25;
            let pixel_color = Color::new(r, g, b);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\nDone.");
}
