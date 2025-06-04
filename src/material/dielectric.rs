use std::ops::Neg;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    util::random_real,
    vec3::{dot, Colour},
};

use super::Material;

#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Colour)> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let ri = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().unit_vector();

        let cos_theta = dot(&unit_direction.neg(), &hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        // || self.reflectance(cos_theta, ri) > random_real()
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_real() {
            unit_direction.reflect(&hit_rec.normal)
        } else {
            unit_direction.refract(&hit_rec.normal, ri)
        };

        Some((Ray::new(hit_rec.p, direction), attenuation))
    }
}
