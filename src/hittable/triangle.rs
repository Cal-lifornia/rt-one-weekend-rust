use std::{rc::Rc, sync::Arc};

use crate::{
    material::Material,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Debug)]
pub struct Triangle {
    verts: [Point3; 3],
    normals: [Point3; 3],
    material: Option<Arc<dyn Material>>,
}

impl Triangle {
    pub fn new(
        verts: [Point3; 3],
        normals: [Point3; 3],
        material: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            verts,
            normals,
            material,
        }
    }
}
