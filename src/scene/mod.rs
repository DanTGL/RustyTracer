use std::sync::Arc;

use crate::math::{Real, Vec3, ray::Ray};

use self::material::{Material, MaterialResult};

pub mod camera;
pub mod material;
pub mod scene;
pub mod sphere;

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: Real,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: Real, t_max: Real) -> Option<HitRecord> {
        None
    }
}
