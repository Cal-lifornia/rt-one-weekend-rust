use std::fmt::Debug;

use crate::{hittable::HitRecord, mod_flat, ray::Ray, vec3::Colour};

mod_flat!(dielectric metal lambertian);

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Colour)>;
}
