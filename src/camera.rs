use crate::{
    ray::Ray,
    util::random_real,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_h: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, image_height: i32) -> Self {
        let mut res = Self {
            aspect_ratio,
            image_width,
            image_height,
            ..Default::default()
        };
        res.initialise();
        res
    }
    pub fn width(&self) -> i32 {
        self.image_width
    }
    pub fn height(&self) -> i32 {
        self.image_height
    }
    pub fn hit_ray(&self, x: usize, y: usize) -> Ray {
        let offset = sample_square();

        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x()) * self.pixel_delta_h)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);
        let ray_direction = pixel_sample - self.centre;
        Ray::new(self.centre, ray_direction)
    }
    fn initialise(&mut self) {
        self.centre = Point3::new(0.0, 0.0, 0.0);

        // Determine viewpoint dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_h = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_h = viewport_h / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.centre - Vec3::new(0.0, 0.0, focal_length) - viewport_h / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_h * self.pixel_delta_v);
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random_real() - 0.5, random_real() - 0.5, 0.0)
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: Default::default(),
            centre: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_h: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }
}
