use crate::{
    camera::Camera,
    colour::Colour,
    grid::Grid,
    hittable::{Hittable, HittableList},
    ray::Ray,
    util::Interval,
};

pub struct Renderer {
    pub camera: Camera,
    pub filename: String,
}

impl Renderer {
    pub fn render_img<F, const W: usize, const H: usize>(
        &self,
        world: HittableList,
        color_hit_by: F,
        mut pixels: Grid<[u8; 3], W, H>,
    ) where
        F: Sync + Send + Fn(&Ray, &HittableList) -> Colour,
    {
        let render_fn = self.render(world, color_hit_by);
        pixels.set_all_parallel(render_fn);
        self.output_img(pixels);
    }

    fn render<F>(
        &self,
        world: HittableList,
        color_hit_by: F,
    ) -> impl Send + Sync + Fn(usize, usize) -> [u8; 3]
    where
        F: Sync + Send + Fn(&Ray, &HittableList) -> Colour,
    {
        let camera = self.camera;
        move |x, y| {
            let ray = camera.hit_ray(x, y);
            color_hit_by(&ray, &world).to_colour_array()
        }
    }

    fn output_img<const W: usize, const H: usize>(&self, pixels: Grid<[u8; 3], W, H>) {
        let mut img_buf = image::ImageBuffer::new(pixels.width() as u32, pixels.height() as u32);
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let colour = pixels.get(x as usize, y as usize);
            *pixel = image::Rgb(*colour);
        }

        img_buf.save(&self.filename).unwrap();
    }
}
pub fn colour_at_ray(r: &Ray, world: &impl Hittable) -> Colour {
    if let Some(hit) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
        return 0.5 * (hit.normal + Colour::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}
