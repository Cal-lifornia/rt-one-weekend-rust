use std::{
    io::{self, Write},
    rc::Rc,
};

use rt_one_weekend::{
    colour::write_colour,
    hittable::HittableList,
    ray::Ray,
    ray_colour,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    let focal_length: f64 = 1.0;
    let camera_centre = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewpoint edges
    let viewport_h = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_h = viewport_h / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_h / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_h + pixel_delta_v);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines Remaining: {} ", IMAGE_HEIGHT - j);
        let mut stderr = std::io::stderr();
        stderr.flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_centre =
                pixel00_loc + (i as f64 * pixel_delta_h) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_centre;
            let ray = Ray::new(camera_centre, ray_direction);
            let pixel_colour = ray_colour(&ray, &mut world);
            write_colour(&mut io::stdout(), &pixel_colour);
        }
    }
    eprintln!("\nDone.")
}
