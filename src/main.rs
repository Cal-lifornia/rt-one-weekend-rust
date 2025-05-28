use rt_one_weekend::{camera::Camera, hittable::HittableList, sphere::Sphere, vec3::Point3};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH);

    cam.render(&mut world);
}
