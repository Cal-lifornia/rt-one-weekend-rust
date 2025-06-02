use std::fmt::Debug;

use crate::{
    camera::Camera,
    grid::Grid,
    hittable::{Hittable, HittableList},
    ray::Ray,
    util::Interval,
    vec3::{Colour, Vec3},
};
use rayon::prelude::*;
use tracing::{debug, info, instrument, span, Level};
use tracing_indicatif::{span_ext::IndicatifSpanExt, style::ProgressStyle};

#[derive(Debug)]
pub struct Renderer {
    pub camera: Camera,
    pub filename: String,
    pub samples: i32,
    pub max_depth: i32,
}

impl Renderer {
    pub fn render_img<F, const W: usize, const H: usize>(
        &self,
        world: HittableList,
        colour_hit_by: F,
        mut pixels: Grid<[u8; 3], W, H>,
    ) where
        F: Sync + Send + Fn(&Ray, i32, &HittableList) -> Colour,
    {
        let render_fn = self.render(world, colour_hit_by);
        pixels.set_all_parallel(render_fn);
        self.output_img(pixels);
    }

    fn render<F>(
        &self,
        world: HittableList,
        colour_hit_by: F,
    ) -> impl Send + Sync + Fn(usize, usize) -> [u8; 3]
    where
        F: Sync + Send + Fn(&Ray, i32, &HittableList) -> Colour,
    {
        let samples = self.samples;
        let camera = self.camera;
        let depth = self.max_depth;
        move |x, y| {
            let sample_rays = (0..samples).into_par_iter().map(|_| {
                let ray = camera.hit_ray(x, y);
                colour_hit_by(&ray, depth, &world)
            });

            let avg_color = sample_rays.sum::<Vec3>() * (1.0 / samples as f64);
            avg_color.to_rgb()
        }
    }

    #[instrument(skip_all)]
    fn output_img<const W: usize, const H: usize>(&self, pixels: Grid<[u8; 3], W, H>) {
        info!("beginning image write");

        let mut img_buf = image::ImageBuffer::new(pixels.width() as u32, pixels.height() as u32);
        let span_header = tracing::info_span!("writing image");
        span_header.pb_set_style(&ProgressStyle::default_bar());
        span_header.pb_set_length((pixels.width() * pixels.height()) as u64);
        let span_header_entered = span_header.enter();
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let colour = pixels.get(x as usize, y as usize);
            *pixel = image::Rgb(*colour);
            span_header.pb_inc(1);
        }

        img_buf.save(&self.filename).expect("writing image");
        std::mem::drop(span_header_entered);
        std::mem::drop(span_header);
    }
}
pub fn colour_at_ray(r: &Ray, depth: i32, world: &impl Hittable) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        let direction = Vec3::random_on_hemisphere(&hit.normal);
        return 0.5 * colour_at_ray(&Ray::new(hit.p, direction), depth - 1, world);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}
