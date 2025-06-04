use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    util::Interval,
    vec3::{dot, Point3, Vec3},
};

#[derive(Clone, Debug)]
pub struct Sphere {
    centre: Point3,
    radius: f64,
    material: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64, material: Option<Arc<dyn Material>>) -> Self {
        Self {
            centre,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.centre - r.origin();
        let a = r.direction().length_squared();
        let h = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit = HitRecord::default();

        hit.t = root;

        hit.p = r.at(hit.t);
        let outward_normal = (hit.p - self.centre) / self.radius;
        hit.set_face_normal(r, &outward_normal);
        hit.material = self.material.clone();

        Some(hit)
    }
}
