use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, Colour, Vec3},
};

use super::Material;

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = ray_in.direction().reflect(&hit_rec.normal).unit_vector()
            + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_rec.p, reflected);
        if dot(&scattered.direction(), &hit_rec.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }
        None
    }
}
