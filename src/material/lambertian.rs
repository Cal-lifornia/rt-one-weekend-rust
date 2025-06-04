use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Colour, Vec3},
};

use super::Material;

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        #[allow(unused_variables)] ray_in: &Ray,
        hit_rec: &HitRecord,
    ) -> Option<(Ray, Colour)> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        Some((Ray::new(hit_rec.p, scatter_direction), self.albedo))
    }
}
