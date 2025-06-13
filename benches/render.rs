use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use rt_one_weekend::{
    camera::{Camera, CameraOptions},
    grid::Grid,
    hittable::{HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    renderer::{ray_colour, Renderer},
    vec3::{Colour, Point3, Vec3},
};

fn run_render() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const FOV: f64 = 20.0;
    const HEIGHT: usize = IMAGE_HEIGHT as usize;
    const WIDTH: usize = IMAGE_WIDTH as usize;

    let mut world = HittableList::new();
    // Ground
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)))),
    ));
    // Centre
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Some(Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)))),
    ));

    // Left
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(Arc::new(Dielectric::new(1.50))),
    ));

    // Bubble
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Some(Arc::new(Dielectric::new(1.00 / 1.50))),
    ));

    // Right
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0))),
    ));

    let cam = Camera::new(&CameraOptions {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        v_fov: FOV,
        look_from: Point3::new(-2.0, 2.0, 1.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 10.0,
        focus_dist: 3.4,
    });

    let pixels: Grid<[u8; 3], WIDTH, HEIGHT> = Default::default();

    let renderer = Renderer {
        camera: cam,
        filename: "output/output.png".into(),
        samples: 100,
        max_depth: 50,
    };

    renderer.render_img(world, ray_colour, pixels);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("software_render");
    bench_group.sample_size(10);
    bench_group.bench_function("render_image", |b| b.iter(|| run_render()));
    bench_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
