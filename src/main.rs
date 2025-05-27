use std::io::{self, Write};

use rt_one_weekend::colour::{write_colour, Colour};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines Remaining: {} ", IMAGE_HEIGHT - j);
        let mut stderr = std::io::stderr();
        stderr.flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_colour = Colour::new(
                (i / IMAGE_WIDTH - 1) as f64,
                (j / IMAGE_WIDTH - 1) as f64,
                0.0,
            );
            write_colour(&mut io::stdout(), &pixel_colour);
        }
    }
    eprintln!("\nDone.")
}
