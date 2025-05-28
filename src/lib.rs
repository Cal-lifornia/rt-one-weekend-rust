use colour::Colour;
use hittable::{HitRecord, Hittable};
use ray::Ray;

pub mod colour;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

pub fn ray_colour(r: &Ray, world: &mut impl Hittable) -> Colour {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}
