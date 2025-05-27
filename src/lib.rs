use colour::Colour;
use ray::Ray;
use vec3::{dot, Point3};

pub mod colour;
pub mod ray;
pub mod vec3;

pub fn hit_sphere(centre: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *centre - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2.0 * dot(r.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discrimnant = (b * b) - (4.0 * a * c);

    discrimnant >= 0.
}
pub fn ray_colour(r: &Ray) -> Colour {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Colour::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}
