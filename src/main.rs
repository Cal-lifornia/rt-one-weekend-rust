use std::{fs::OpenOptions, sync::Arc};

use rand::Rng;
use rt_one_weekend::{
    camera::{Camera, CameraOptions},
    grid::Grid,
    hittable::{HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    renderer::{ray_colour, Renderer},
    util::random_real,
    vec3::{Colour, Point3, Vec3},
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{
    layer::{Filter, SubscriberExt},
    Layer,
};

fn main() {
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

    #[cfg(feature = "software_render")]
    software_render();

    #[cfg(feature = "gpu_render")]
    match rt_one_weekend::state::run() {
        Ok(_) => {}
        Err(e) => tracing::error!("ran into error {}", e),
    }
}

fn software_render() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const FOV: f64 = 20.0;
    const HEIGHT: usize = IMAGE_HEIGHT as usize;
    const WIDTH: usize = IMAGE_WIDTH as usize;

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(ground_material),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_real();
            let centre = Point3::new(
                a as f64 + 0.9 + random_real(),
                0.2,
                b as f64 + 0.9 * random_real(),
            );
            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.0 {
                    let albedo = Colour::random_real() * Colour::random_real();
                    let material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(centre, 0.2, Some(material)));
                } else if choose_mat < 0.95 {
                    let albedo = Colour::new(
                        rand::rng().random_range(0.5..=1.0),
                        rand::rng().random_range(0.5..=1.0),
                        rand::rng().random_range(0.5..=1.0),
                    );
                    let fuzz = rand::rng().random_range(0.0..=0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(centre, 0.2, Some(material)));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(centre, 0.2, Some(material)));
                };
            }
        }
    }

    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Some(Arc::new(Dielectric::new(1.5))),
    ));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)))),
    ));
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Some(Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0))),
    ));

    let cam = Camera::new(&CameraOptions {
        aspect_ratio: ASPECT_RATIO,
        image_width: IMAGE_WIDTH,
        image_height: IMAGE_HEIGHT,
        v_fov: FOV,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    });

    let pixels: Grid<[u8; 3], WIDTH, HEIGHT> = Default::default();

    let renderer = Renderer {
        camera: cam,
        filename: "output/output.png".into(),
        samples: 500,
        max_depth: 50,
    };

    renderer.render_img(world, ray_colour, pixels);
}

struct DebugOnlyFilter;
impl<S> Filter<S> for DebugOnlyFilter {
    fn enabled(
        &self,
        meta: &tracing::Metadata<'_>,
        _cx: &tracing_subscriber::layer::Context<'_, S>,
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
        _cx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        meta.level() == &Level::ERROR
    }
}
