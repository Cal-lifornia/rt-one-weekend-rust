use std::{f64::consts::PI, fs::OpenOptions, sync::Arc};

use rt_one_weekend::{
    camera::{Camera, CameraOptions},
    grid::Grid,
    hittable::{HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    renderer::{ray_colour, Renderer},
    vec3::{Colour, Point3, Vec3},
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{
    layer::{Filter, SubscriberExt},
    Layer,
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 500;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const FOV: f64 = 20.0;
    const HEIGHT: usize = IMAGE_HEIGHT as usize;
    const WIDTH: usize = IMAGE_WIDTH as usize;

    let err_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log-error.log")
        .unwrap();
    let debug_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log-debug.log")
        .unwrap();

    let indicatif_layer = IndicatifLayer::new();

    let subscriber = tracing_subscriber::registry()
        .with(indicatif_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(true)
                .with_filter(LevelFilter::from_level(Level::INFO)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(debug_file)
                .with_filter(DebugOnlyFilter),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(err_file)
                .with_filter(ErrorOnlyFilter),
        );
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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

struct DebugOnlyFilter;
impl<S> Filter<S> for DebugOnlyFilter {
    fn enabled(
        &self,
        meta: &tracing::Metadata<'_>,
        #[allow(unused_variables)] cx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        meta.level() == &Level::DEBUG
    }
}
#[allow(unused_variables)]
struct ErrorOnlyFilter;
impl<S> Filter<S> for ErrorOnlyFilter {
    fn enabled(
        &self,
        meta: &tracing::Metadata<'_>,
        #[allow(unused_variables)] cx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        meta.level() == &Level::ERROR
    }
}
