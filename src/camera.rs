use std::io::{self, Write};

use crate::{
    colour::{write_colour, Colour},
    hittable::{HitRecord, Hittable},
    ray::Ray,
    util::Interval,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_h: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            ..Default::default()
        }
    }
    pub fn ray_colour(r: &Ray, world: &mut impl Hittable) -> Colour {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }
    pub fn render(&mut self, world: &mut impl Hittable) {
        self.initialise();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines Remaining: {} ", self.image_height - j);
            let mut stderr = std::io::stderr();
            stderr.flush().unwrap();
            for i in 0..self.image_width {
                let pixel_centre = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_h)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_centre - self.centre;
                let ray = Ray::new(self.centre, ray_direction);
                let pixel_colour = Self::ray_colour(&ray, world);
                write_colour(&mut io::stdout(), &pixel_colour);
            }
        }
        eprintln!("\nDone.")
    }
    fn initialise(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
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
