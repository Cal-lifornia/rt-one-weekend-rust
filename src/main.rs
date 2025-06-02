use std::fs::OpenOptions;

use rt_one_weekend::{
    camera::Camera,
    grid::Grid,
    hittable::HittableList,
    renderer::{colour_at_ray, Renderer},
    sphere::Sphere,
    vec3::Point3,
};
use tracing::{level_filters::LevelFilter, Level};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{
    layer::{Filter, SubscriberExt},
    Layer,
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
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
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::new(ASPECT_RATIO, IMAGE_WIDTH, IMAGE_HEIGHT);

    let pixels: Grid<[u8; 3], WIDTH, HEIGHT> = Default::default();

    let renderer = Renderer {
        camera: cam,
        filename: "output/output.png".into(),
        samples: 100,
        max_depth: 50,
    };

    renderer.render_img(world, colour_at_ray, pixels);
}

struct DebugOnlyFilter;
impl<S> Filter<S> for DebugOnlyFilter {
    fn enabled(
        &self,
        meta: &tracing::Metadata<'_>,
        cx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        meta.level() == &Level::DEBUG
    }
}
struct ErrorOnlyFilter;
impl<S> Filter<S> for ErrorOnlyFilter {
    fn enabled(
        &self,
        meta: &tracing::Metadata<'_>,
        cx: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        meta.level() == &Level::ERROR
    }
}
