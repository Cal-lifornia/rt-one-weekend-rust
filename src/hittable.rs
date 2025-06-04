use std::{fmt::Debug, ops::Neg, sync::Arc};

use crate::{
    material::Material,
    ray::Ray,
    util::Interval,
    vec3::{dot, Point3, Vec3},
};

#[derive(Default, Clone, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    /// Sets the HitRecord normal vector.
    /// NOTE: The parameter 'outward_normal' is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            outward_normal.neg()
        }
    }
}

pub trait Hittable: Sync + Send + Debug {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Default, Debug)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(res) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = res.t;
                result = Some(res);
            };
        }

        result
    }
}
