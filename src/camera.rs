use crate::{
    ray::Ray,
    util::{degrees_to_radians, random_real},
    vec3::{cross, Point3, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub v_fov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Vec3,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_h: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        image_height: i32,
        v_fov: f64,
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
    ) -> Self {
        let mut res = Self {
            aspect_ratio,
            image_width,
            image_height,
            v_fov,
            look_from,
            look_at,
            v_up,
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
        self.centre = self.look_from;

        // Determine viewpoint dimensions
        let focal_length = (self.look_from - self.look_at).length();
        let theta = degrees_to_radians(self.v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = (cross(&self.v_up, &self.w)).unit_vector();
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport
        // edges
        let viewport_h = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_h = viewport_h / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.centre - (focal_length * self.w) - viewport_h / 2.0 - viewport_v / 2.0;
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
            v_fov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            image_height: Default::default(),
            centre: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_h: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
        }
    }
}
