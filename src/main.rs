const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let g: f32 = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
